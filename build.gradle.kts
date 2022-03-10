import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "1.6.0"
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
    testImplementation("dev.kord:kord-core:0.8.x-SNAPSHOT")

    testImplementation("com.github.natanbc:lavadsp:0.7.7")
    testImplementation("com.github.natanbc:native-loader:0.7.2")

}

java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}

sourceSets {
    create("rust") {
    }
}

tasks.create<Exec>("buildRust") {
    workingDir = file("src/main/rust")
    commandLine = listOf("cargo", "build", "--release")

    copy {
        from("build/rust/release/libconnector.so")
        into("src/main/resources/natives/linux-x86-64")
    }
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = "1.8"
}
