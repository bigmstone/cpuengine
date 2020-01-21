# CPU Renderer
Several months ago I started on a dual-path journey. To learn Rust, and to learn
more about graphics rendering. This is the result of that initial step. I since
abandoned work on this CPU renderer to work on a real-time game engine (which
was always the end goal) but I thought this still has some interesting aspects.
Since you're not interacting with a GPU at all, and take advantage of none of
the tools available to you when you do rendering from the GPU, this was an
interesting project to teach me a bit more about what's actually going on at a
lower more fundamental level. There's a million and one way this could be
improved and enhanced (such as CPU rendered lighting, or CPU based PBR) but I'm
focusing those efforts on the real-time engine. I might come back to this and
implement a couple of different features if it proves a useful to do an initial
run at something.

I haven't re-checked this code no 10+ months into Rust. I'm sure there's demons
in there ready to bite you.

Almost all of this was built following [ssloy's
wiki](https://github.com/ssloy/tinyrenderer/wiki)
