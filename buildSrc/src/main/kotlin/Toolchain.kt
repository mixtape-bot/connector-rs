data class Toolchain(
    val platform: String,
    val arch: String,
    val extension: String,
    val suffix: String,
    val rustTarget: String,
) {
    val name       = "${platform.split('-').joinToString("") { it.capitalize() }}${arch.capitalize()}"
    val buildPath  = "$rustTarget/release/${suffix}connector.$extension"
    val destFolder = "natives/$platform${if (arch.isNotBlank()) "-$arch" else ""}"
}
