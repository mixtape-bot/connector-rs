import com.sedmelluq.discord.lavaplayer.natives.ConnectorNativeLibLoader
import com.sedmelluq.discord.lavaplayer.player.AudioConfiguration
import com.sedmelluq.discord.lavaplayer.player.DefaultAudioPlayerManager
import com.sedmelluq.discord.lavaplayer.player.FunctionalResultHandler
import com.sedmelluq.discord.lavaplayer.player.event.TrackEndEvent
import com.sedmelluq.discord.lavaplayer.source.AudioSourceManagers
import com.sedmelluq.discord.lavaplayer.track.AudioTrack
import dev.kord.common.annotation.KordVoice
import dev.kord.common.ratelimit.BucketRateLimiter
import dev.kord.core.Kord
import dev.kord.core.behavior.reply
import dev.kord.core.event.gateway.ReadyEvent
import dev.kord.core.event.message.MessageCreateEvent
import dev.kord.core.on
import dev.kord.gateway.DefaultGateway
import dev.kord.gateway.Intent
import dev.kord.gateway.Intents
import dev.kord.rest.builder.message.create.embed
import dev.kord.voice.AudioFrame
import dev.kord.voice.AudioProvider
import gg.mixtape.natives.connector.ConnectorDebugLibrary
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.runBlocking
import mu.KotlinLogging
import java.lang.management.ManagementFactory
import kotlin.coroutines.resume
import kotlin.coroutines.suspendCoroutine
import kotlin.text.Typography.bullet
import kotlin.time.Duration.Companion.seconds

val log = KotlinLogging.logger { }

@OptIn(DelicateCoroutinesApi::class, KordVoice::class)
suspend fun main() {
    ConnectorNativeLibLoader.loadConnectorLibrary()
    ConnectorDebugLibrary.configureLogging()

    val kord = Kord(System.getenv("BOT_TOKEN")) {
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

    val players = DefaultAudioPlayerManager()

    players.configuration.apply {
        resamplingQuality = AudioConfiguration.ResamplingQuality.HIGH
    }

    AudioSourceManagers.registerRemoteSources(players)
    AudioSourceManagers.registerLocalSource(players)

    kord.on<MessageCreateEvent> {
        if (message.author?.isBot != false || !message.content.startsWith("!/")) {
            return@on
        }

        val args = message.content
            .drop(2)
            .split(" +".toRegex())
            .toMutableList()

        when (val command = args.removeFirst()) {
            "play" -> {
                val query = args.joinToString(" ")

                /* load the item */
                val track = suspendCoroutine<AudioTrack?> { cont ->
                    players.loadItem(query, FunctionalResultHandler(
                        { cont.resume(it) },
                        { cont.resume(it.tracks.first()) },
                        { cont.resume(null) },
                        { cont.resume(null) },
                    ))
                } ?: return@on

                val player = players.createPlayer()

                /*player.on<TrackStartEvent> {
                    println("waiting timescale")
                    delay(5000)

                    println("setting timescale")
                    player.setFilterFactory { _, format, output ->
                        val timescale = TimescalePcmAudioFilter(output, format.channelCount, format.sampleRate)
                        timescale.pitch = 1.1

                        val filter: FloatPcmAudioFilter = ResamplingPcmAudioFilter(
                            players.configuration,
                            format.channelCount,
                            timescale,
                            format.sampleRate,
                            (format.sampleRate / 1.125).toInt()
                        )

                        listOf(filter)
                    }
                }*/

                /* join vc */
                val vc = message.getAuthorAsMember()?.getVoiceStateOrNull()?.getChannelOrNull()
                    ?: return@on

                val connection = vc.connect {
                    audioProvider = AudioProvider { AudioFrame.fromData(player.provide()?.data) }
                }

                player.addListener {
                    if (it is TrackEndEvent) {
                        runBlocking { connection.shutdown() }
                        player.destroy()
                    }
                }

                player.playTrack(track)

                message.reply {
                    embed { description = "Now playing [**${track.info.title}**](${track.info.uri})" }
                }
            }

            "info" -> message.reply {
                val runtime = Runtime.getRuntime()
                val usage = runtime.totalMemory() - runtime.freeMemory()

                embed {
                    description = buildString {
                        appendLine("$bullet Threads: ${ManagementFactory.getThreadMXBean().threadCount}")
                        appendLine("$bullet Memory Usage: ${usage / 1024 / 1024} MB")
                    }
                }
            }

            else -> message.reply {
                embed { description = "Never heard of a command with the name: **$command**" }
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
