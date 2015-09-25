use std::io::Write;
use std::string::String;
use audio::{AudioData,AudioFile};
use wav_format::WavFormat;

//Structs

struct FileSystem;
struct AisoAdapter;


//Traits
trait OutputHandler {
	fn convert(audioData: &AudioData) -> AudioData;
	fn output(audioData: &AudioData) -> Write;
}


//Implementation

fn output<T: OutputHandler>(handler: T) {
	
}


impl OutputHandler for FileSystem {
	fn convert(audioData: &AudioData) {
		let fmt : AudioFile = WavFormat {
			channels: 1,
			sampleLen: 1,
			sampleRate: 44100,
		};
		
		let audioFileData = fmt.build(&audioData);
		
		return &audioFileData;
	}
	
	fn output(audioData: &AudioData) {
		let buf : Write = Write::new();
		return buf;
	}
}