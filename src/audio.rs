use std::boxed::Box;

//Structs

struct AudioData {
	data: Box<[u8]>,
}



//Traits

trait AudioFile {
	fn build(&self, &AudioData) -> &AudioData;
	fn getSamples(&self, &AudioData) -> u32;
}

trait Modifier {
	fn modify(&self, &audioData : AudioData);
}



//Implementation

impl AudioData {
	fn getData(&self) {
		return &self.data;
	}
}

