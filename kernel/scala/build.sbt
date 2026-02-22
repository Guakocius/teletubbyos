import scala.scalanative.build.*

enablePlugins(ScalaNativePlugin)

scalaVersion := "3.3.3"

nativeConfig ~= { c =>
  c.withMode(Mode.releaseFull)
   .withGC(GC.none)           // no GC �� bare metal!
   .withLTO(LTO.full)
   .withOptimize(true)
   .withLinkingOptions(Seq(
     "-nostdlib",
     "-static",
     "-Wl,-T../../kernel/linker.ld",
     "-Wl,--no-dynamic-linker",
   ))
   .withCompileOptions(Seq(
     "-ffreestanding",
     "-fno-stack-protector",
     "-mno-red-zone",
   ))
}
