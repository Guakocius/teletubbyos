import scala.scalanative.build.*
import java.nio.file.Paths

enablePlugins(ScalaNativePlugin)

scalaVersion := "3.3.3"

// Resolve clang from PATH, falling back to common install locations
def findClang(name: String): java.nio.file.Path = {
  val fromPath = scala.util.Try(
    sys.process.Process(Seq("which", name)).!!.trim
  ).toOption.filter(_.nonEmpty)
  val candidates = Seq(
    fromPath,
    Some(s"/usr/bin/$name"),
    Some(s"/usr/local/bin/$name"),
    Some(s"/run/current-system/sw/bin/$name"), // NixOS
  ).flatten
  candidates
    .map(Paths.get(_))
    .find(p => p.toFile.exists && p.toFile.canExecute)
    .getOrElse(Paths.get(name)) // last resort: let the OS find it
}

nativeConfig := {
  // build.sbt lives in kernel/scala/ — linker.ld is one level up in kernel/
  val linkerScript = (ThisBuild / baseDirectory).value / ".." / "linker.ld"
  NativeConfig.empty
    .withMode(Mode.releaseFull)
    .withGC(GC.none)
    .withLTO(LTO.full)
    .withOptimize(true)
    .withClang(findClang("clang"))
    .withClangPP(findClang("clang++"))
    .withLinkingOptions(Seq(
      "-nostdlib",
      "-static",
      s"-Wl,-T,${linkerScript.getCanonicalPath}",
      "-Wl,--no-dynamic-linker",
    ))
    // NOTE: Do NOT put -ffreestanding here — it breaks Scala Native's own
    // C runtime (nativelib/gc.c, libunwind) which needs standard libc headers.
    .withCompileOptions(Seq(
      "-fno-stack-protector",
      "-mno-red-zone",
    ))
}