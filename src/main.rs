#![allow(dead_code)]

use std::prelude::*;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::mem;

fn main() {
	let mut bucket : Vec<String> = Vec::new();
	
}

struct Drop {
	events: Vec<EventType>,
	emotives: Vec<EmotiveType>,
	transitions: TransitionSet,
}

struct TransitionSet {
	preTransition: Transition,
	postTransition: Transition,
}

//IO

fn output<T: OutputHandler>(handler: T) {
	
}

trait OutputHandler {
	fn convert(audioData: &AudioData) -> &[u8];
	fn outTo() -> Write;
}

struct FileSystem {
}

impl OutputHandler for FileSystem {
	fn convert(audioData: &AudioData) {
		WavFile::convertTo(audioData);
	}
}

//Audio Files

struct RawAudio {
	data: Box::<[u8]>;
}

impl RawAudio {
	fn getData(&self) {
		return &self.data;
	}
}

trait AudioFile {
	fn build() -> &[u8];
}

struct WavFormat {
	samples: u32,
	channels: u8,
	sampleLen: u8,
	sampleRate: u32,
}

impl AudioFile for WavFormat {
	fn build(&self, &AudioData) {
		let mut data : Vec<u8> = Vec::new();
		
		//Meta format
		data.push_all("RIFF".as_bytes());
		
		let fileSize : &[i8; 4] = unsafe { mem::transmute(36 + self.samples * self.channels * self.sampleLen) };
		data.push_all(fileSize);
		
		//Audio format
		data.push_all("WAVEfmt ".as_bytes());
		
		//Wave section chunk size
		let wavSecSize : &[i8; 4] = unsafe { mem::transmute(16) };
		data.push_all(wavSecSize);
		
		let wavFmtType : &[i8; 2] = unsafe { mem::transmute(1_i16) };
		data.push_all(wavFmtType);
		
		//Mono or Stereo
		let monoStereo : &[i8; 2] = unsafe { mem::transmute(self.channels as i16) };
		data.push_all(monoStereo);
		

		let sampleRate : &[i8; 4] = unsafe { mem::transmute(self.sampleRate) };
		data.push_all(sampleRate);
		
		//Bytes Per Second
		let bytesSec : &[i8; 4] = unsafe { mem::transmute(self.sampleRate * self.sampleLen * self.channels) };
		data.push_all(bytesSec);
		
		//Block Alignment
		let blockAlign : &[i8; 2] = unsafe { mem::transmute((self.sampleLen * self.channels) as i16) };
		data.push_all(blockAlign);
		
		//Bits Per Sample
		let bitSmpl : &[i8; 2] = unsafe { mem::transmute((8 * self.sampleLen) as i16) };
		data.push_all(bitSmpl);
		
		//Data Header
		data.push_all("data".as_bytes());
		
		//Data Chunk Size
		let chunkSize : &[i8; 4] = unsafe { mem::transmute(self.samples * self.sampleLen) };
		data.push_all(chunkSize);
		
		//Finally, append the data
		for byte in audioData.getData().bytes() {
			data.push(byte);
		}
		
		return data.as_slice();
	}
}


// amt = Amount, vlt = Volatility, wt = Weight

struct EvMeta {
	amt: i32,
	vlt: i32,
	wt: i32
}

enum EventType {
	Action(EvMeta),
	Calm(EvMeta),
	Interjection(EvMeta),
	Suspense(EvMeta),
	Clarity(EvMeta),
	Levity(EvMeta),
	Humble(EvMeta),
	Uncertainty(EvMeta),
	Purity(EvMeta),
	Evil(EvMeta),
	Light(EvMeta),
	Dark(EvMeta),
	Expected(EvMeta),
	Anomalous(EvMeta),
	Mysterious(EvMeta),
}

struct EmMeta {
	amt: i32,
	vlt: i32,
	wt: i32
}

enum EmotiveType {
	Joy(EmMeta),
	Surprise(EmMeta),
	Sadness(EmMeta),
	Fear(EmMeta),
	Anger(EmMeta),
	Craze(EmMeta),
	Angst(EmMeta),
}

struct TnMeta {
	amt: i32,
	dir: TransitionDir,
	sp: TransitionSpeed,
}

enum Transition {
	Hard(TnMeta),
	Soft(TnMeta),
	Staticy(TnMeta),
	Smooth(TnMeta),
	Rocky(TnMeta),
	Glitchy(TnMeta),
}

enum TransitionDir {
	Pre,
	Post,
}

enum TransitionSpeed {
	Instant,
	Fast,
	Medium,
	Slow,
	Lagging,
}