# titleformat

**titleformat** is a title formatting library for media files written in Rust. The aim of the project is to be conformant to the foobar2000 formatting syntax.

For more reference about the Foobar syntax, please refer to http://wiki.hydrogenaud.io/index.php?title=Foobar2000:Title_Formatting_Reference

To try out the formatting features, compile the crate with `cargo build --features=ffmpeg` and use the `titleformatter` executable with a formatting expression as first argument, such as `"%track%. %title%"`, and the audio files you want to apply the title formatting to as subsequent arguments.

Example: `$ titleformatter "%track%. [%artist% - ] %title%" test01.flac test02.mp3 music/*`

## Currently implemented functionalities
* Field references, such as %artist% or %title%
* Conditional selection with square brackets [ ] for optional sub-expressions