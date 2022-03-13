@file:OptIn(KordVoice::class)

import com.github.natanbc.lavadsp.timescale.TimescalePcmAudioFilter
import com.sedmelluq.discord.lavaplayer.filter.FloatPcmAudioFilter
import com.sedmelluq.discord.lavaplayer.filter.ResamplingPcmAudioFilter
import com.sedmelluq.discord.lavaplayer.player.AudioConfiguration
import com.sedmelluq.discord.lavaplayer.player.DefaultAudioPlayerManager
import com.sedmelluq.discord.lavaplayer.player.FunctionalResultHandler
import com.sedmelluq.discord.lavaplayer.source.AudioSourceManagers
import com.sedmelluq.discord.lavaplayer.track.AudioTrack
import dev.kord.common.Color
import dev.kord.common.annotation.KordVoice
import dev.kord.common.entity.Snowflake
import dev.kord.common.ratelimit.BucketRateLimiter
import dev.kord.core.Kord
import dev.kord.core.behavior.channel.createEmbed
import dev.kord.core.event.gateway.ReadyEvent
import dev.kord.core.event.message.MessageCreateEvent
import dev.kord.core.on
import dev.kord.gateway.DefaultGateway
import dev.kord.gateway.Intent
import dev.kord.gateway.Intents
import dev.kord.gateway.Ticker
import dev.kord.rest.builder.message.EmbedBuilder
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import mu.KotlinLogging
import java.lang.management.ManagementFactory
import kotlin.coroutines.resume
import kotlin.coroutines.suspendCoroutine
import kotlin.text.Typography.bullet
import kotlin.time.Duration.Companion.seconds

const val PRIMARY_COLOR = 0xB963A5

val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())
val log = KotlinLogging.logger { }
val apm = DefaultAudioPlayerManager()
val players = mutableMapOf<Snowflake, Player>()

lateinit var kord: Kord

suspend fun main() {
    kord = Kord(System.getenv("BOT_TOKEN")) {
        gateways { _, shards ->
            val rateLimiter = BucketRateLimiter(1, 5.seconds)
            shards.map {
                DefaultGateway {
                    url = "wss://gateway.discord.gg/?v=9&encoding=json&compress=zlib-stream"
                    identifyRateLimiter = rateLimiter
                }
            }
        }
    }

    scope.launch {
        Ticker(Dispatchers.Default).tickAt(5000) {
            kord.unsafe.messageChannel(Snowflake(830270945213284403)).createEmbed {
                applyInfoEmbed()
            }
        }
    }

    apm.configuration.apply {
        resamplingQuality = AudioConfiguration.ResamplingQuality.HIGH
    }

    AudioSourceManagers.registerRemoteSources(apm)
    AudioSourceManagers.registerLocalSource(apm)

    kord.on<MessageCreateEvent> {
        if (message.author?.isBot != false || !message.content.startsWith("!/")) {
            return@on
        }

        val guildId = message.data.guildId.value ?: return@on
        val args = message.content.drop(2).split(" +".toRegex()).toMutableList()

        when (val command = args.removeFirst()) {
            "join" -> {
                val player = Player.create(message) ?: return@on

                message.replyEmbed {
                    description = "Joined ${player.voiceChannel?.mention}"
                    color = Color(PRIMARY_COLOR)
                }
            }

            "play" -> {
                val query = args.joinToString(" ")

                /* load the item */
                val track = suspendCoroutine<AudioTrack?> { cont ->
                    apm.loadItem(query,
                        FunctionalResultHandler(
                            { cont.resume(it) },
                            { cont.resume(it.tracks.first()) },
                            { cont.resume(null) },
                            { cont.resume(null) },
                        ))
                } ?: return@on

                val player = Player.create(message) ?: return@on

                player.audioPlayer.startTrack(track, false)
            }

            "nightcore" -> {
                val player = players[guildId] ?: return@on

                player.audioPlayer.setFilterFactory { track, format, output ->
                    val timescale = TimescalePcmAudioFilter(output, format.channelCount, format.sampleRate)
                    timescale.pitch = 1.2

                    val filter: FloatPcmAudioFilter = ResamplingPcmAudioFilter(
                        apm.configuration,
                        format.channelCount,
                        timescale,
                        format.sampleRate,
                        (format.sampleRate / 1.125).toInt()
                    )

                    listOf(filter)
                }
            }

            "info" -> message.replyEmbed {
                applyInfoEmbed()
            }

            else -> message.replyEmbed {
                description = "Never heard of a command with the name: **$command**"
                color = Color(PRIMARY_COLOR)
            }
        }
    }

    kord.on<ReadyEvent> {
        log.info { "Now ready as ${self.tag}" }
    }

    kord.login {
        intents = Intents {
            +Intent.Guilds
            +Intent.GuildMessages
            +Intent.GuildVoiceStates
        }
    }
}

fun EmbedBuilder.applyInfoEmbed() {
    val runtime = Runtime.getRuntime()
    description = buildString {
        appendLine("$bullet Threads: ${ManagementFactory.getThreadMXBean().threadCount}")
        appendLine("$bullet Memory Usage: ${(runtime.totalMemory() - runtime.freeMemory()) / 1024 / 1024} MB")
    }

    color = Color(PRIMARY_COLOR)
}
