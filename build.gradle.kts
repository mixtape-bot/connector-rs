import org.apache.tools.ant.taskdefs.condition.Os
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm")
}

group = "gg.mixtape"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
    maven("https://m2.dv8tion.net/releases")
    maven("https://dimensional.jfrog.io/artifactory/maven")
    maven("https://oss.sonatype.org/content/repositories/snapshots")
    maven("https://jitpack.io")
    mavenLocal()
}

dependencies {
    testImplementation("com.github.walkyst:lavaplayer-fork:1.3.97") {
        exclude(group = "com.sedmelluq", module = "lavaplayer-natives")
    }

    testImplementation("ch.qos.logback:logback-classic:1.2.11")
    testImplementation("dev.kord:kord-core:0.8.0-M11")

    testImplementation("com.github.natanbc:lavadsp:0.7.7")
    testImplementation("com.github.natanbc:native-loader:0.7.2")
}

java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}


Toolchains.ALL.forEach { toolchain ->
    tasks.register<Exec>("compileRust${toolchain.name}") {
        commandLine = listOf("cargo", "build", "--release", "--target", toolchain.rustTarget)

        /* deploy the native files */
        copy {
            from("build/rust/${toolchain.buildPath}")
            into("src/main/resources/${toolchain.destFolder}")
        }
    }

    /*tasks.register<Copy>("deployRust${toolchain.name}") {
        copy {
            from("build/rust/${toolchain.buildPath}")
            into("src/main/resources/${toolchain.destFolder}")
        }
    }*/
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = "1.8"
}
