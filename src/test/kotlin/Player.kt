import com.sedmelluq.discord.lavaplayer.player.AudioPlayer
import com.sedmelluq.discord.lavaplayer.player.event.AudioEvent
import com.sedmelluq.discord.lavaplayer.player.event.AudioEventListener
import com.sedmelluq.discord.lavaplayer.player.event.TrackEndEvent
import com.sedmelluq.discord.lavaplayer.player.event.TrackStartEvent
import com.sedmelluq.discord.lavaplayer.track.AudioTrack
import com.sedmelluq.discord.lavaplayer.track.AudioTrackEndReason
import dev.kord.common.Color
import dev.kord.common.annotation.KordVoice
import dev.kord.common.entity.Snowflake
import dev.kord.common.entity.optional.OptionalSnowflake
import dev.kord.common.entity.optional.optionalSnowflake
import dev.kord.core.behavior.channel.MessageChannelBehavior
import dev.kord.core.behavior.channel.TextChannelBehavior
import dev.kord.core.behavior.channel.VoiceChannelBehavior
import dev.kord.core.behavior.channel.createEmbed
import dev.kord.core.entity.Message
import dev.kord.voice.AudioFrame
import dev.kord.voice.AudioProvider
import dev.kord.voice.encryption.strategies.LiteNonceStrategy
import kotlinx.coroutines.launch

class Player(val guildId: Snowflake) : AudioEventListener {
    companion object {
        suspend fun create(message: Message, join: Boolean = true): Player? {
            val guild = message.getGuild().id
            if (players.containsKey(guild)) {
                return players[guild]
            }

            val player = Player(guild)
            player.textChannelId = message.channelId.optionalSnowflake()

            if (join) {
                val vc = message.getAuthorAsMember()?.getVoiceStateOrNull()?.channelId
                    ?: return null

                player.join(vc)
            }

            players[guild] = player
            return player
        }
    }

    val audioPlayer: AudioPlayer by lazy {
        apm.createPlayer().also { it.addListener(this) }
    }

    var voiceChannelId: OptionalSnowflake = OptionalSnowflake.Missing
    val voiceChannel: VoiceChannelBehavior?
        get() = voiceChannelId.value?.let {
            VoiceChannelBehavior(guildId,
                it,
                kord)
        }

    var textChannelId: OptionalSnowflake = OptionalSnowflake.Missing
    val textChannel: MessageChannelBehavior? get() = textChannelId.value?.let { TextChannelBehavior(guildId, it, kord) }

    @OptIn(KordVoice::class)
    suspend fun join(vc: Snowflake): Player {
        voiceChannelId = vc.optionalSnowflake()

        requireNotNull(voiceChannel).connect {
            audioProvider = AudioProvider { AudioFrame.fromData(audioPlayer.provide()?.data) }
            nonceStrategy = LiteNonceStrategy()
        }

        return this
    }

    private suspend fun onTrackStart(track: AudioTrack) {
        textChannel?.createEmbed {
           description = "Now playing [**${track.info.title}**](${track.info.uri})"
            color = Color(PRIMARY_COLOR)
        }
    }

    private suspend fun onTrackEnd(track: AudioTrack, endReason: AudioTrackEndReason) {
        textChannel?.createEmbed {
            description = "The track has stopped."
            color = Color(PRIMARY_COLOR)
        }
    }

    override fun onEvent(event: AudioEvent) {
        scope.launch {
            when (event) {
                is TrackStartEvent -> onTrackStart(event.track)
                is TrackEndEvent -> onTrackEnd(event.track, event.endReason)
            }
        }
    }
}
