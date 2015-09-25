use std::mem::transmute;
use std::vec::Vec;
use audio::{AudioData,AudioFile};


//Structs

struct WavFormat {
	channels: u8,  //1 for mono, 2 for stereo
	sampleLen: u8, //in bytes
	sampleRate: u32,
}

//Implementation

impl AudioFile for WavFormat {
	fn getSamples(&self, audioData: &AudioData) {
		return (audioData.getData().downcast::<[u8]>().len() / self.sampleLen).floor();
	}

	fn build(&self, audioData: &AudioData) {
		let mut data : Vec<u8> = Vec::new();
		let samples = self.getSamples(&audioData);
		
		//Meta format
		data.push_all("RIFF".as_bytes());
		
		let fileSize : &[i8; 4] = unsafe { transmute(36 + samples * self.channels * self.sampleLen) };
		data.push_all(fileSize);
		
		//Audio format
		data.push_all("WAVEfmt ".as_bytes());
		
		//Wave section chunk size
		let wavSecSize : &[i8; 4] = unsafe { transmute(16) };
		data.push_all(wavSecSize);
		
		let wavFmtType : &[i8; 2] = unsafe { transmute(1_i16) };
		data.push_all(wavFmtType);
		
		//Mono or Stereo
		let monoStereo : &[i8; 2] = unsafe { transmute(self.channels as i16) };
		data.push_all(monoStereo);
		

		let sampleRate : &[i8; 4] = unsafe { transmute(self.sampleRate) };
		data.push_all(sampleRate);
		
		//Bytes Per Second
		let bytesSec : &[i8; 4] = unsafe { transmute(self.sampleRate * self.sampleLen * self.channels) };
		data.push_all(bytesSec);
		
		//Block Alignment
		let blockAlign : &[i8; 2] = unsafe { transmute((self.sampleLen * self.channels) as i16) };
		data.push_all(blockAlign);
		
		//Bits Per Sample
		let bitSmpl : &[i8; 2] = unsafe { transmute((8 * self.sampleLen) as i16) };
		data.push_all(bitSmpl);
		
		//Data Header
		data.push_all("data".as_bytes());
		
		//Data Chunk Size
		let chunkSize : &[i8; 4] = unsafe { transmute(samples * self.sampleLen) };
		data.push_all(chunkSize);
		
		//Finally, append the data
		for byte in audioData.getData().bytes() {
			data.push(byte);
		}
		
		return AudioData { data: Box::<[u8]>::new(data.as_slice()) };
	}
}