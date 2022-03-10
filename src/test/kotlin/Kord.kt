import dev.kord.core.behavior.MessageBehavior
import dev.kord.core.behavior.reply
import dev.kord.core.entity.Message
import dev.kord.rest.builder.message.EmbedBuilder
import dev.kord.rest.builder.message.create.embed
import kotlin.contracts.ExperimentalContracts
import kotlin.contracts.InvocationKind
import kotlin.contracts.contract

@OptIn(ExperimentalContracts::class)
suspend inline fun MessageBehavior.replyEmbed(build: EmbedBuilder.() -> Unit): Message {
    contract { callsInPlace(build, InvocationKind.EXACTLY_ONCE) }

    return reply { embed(build) }
}
