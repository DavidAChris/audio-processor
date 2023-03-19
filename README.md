AUDIO-PROCESSOR
Disclaimer: This is not completed Software. This was simply a prototype to remove all silence from within a wav file.
Note: For any performance benchmarking the application should be run in release mode.
simply add --release to any cargo command.

> To test the program you will need to provide your own .wav files as Github does not allow hosting large files.
> next simply run cargo test --release
> After running the tests you can check the 'WavSilenceRemoval.log' file to check how quickly the program ran
> for each wav file. 

WAVE AUDIO FORMAT
```
13 Headers
Total Bytes | Byte Positions | Example Value
    4       |     1-4        |    RIFF 
    4       |     5-8        |    File Size 8bit int
    4       |     9-12       |    "WAVE"
    4       |     13-16      |    "fmt "
    4       |     17-20      |    16 Length of format data
    2       |     21-22      |    1 (PMC) 2byte int
    2       |     23-24      |    2 Num of channels
    4       |     25-28      |    44100 Sample Rate (44100 (CD), 48000 (DAT). Sample Rate = Number of Samples per second, or Hertz)
    4       |     29-32      |    176400 (Sample Rate * BitsPerSample * Channels) / 8
    2       |     33-34      |    4 (BitsPerSample * Channels) / 8.1 - 8 bit mono2 - 8 bit stereo/16 bit mono4 - 16 bit stereo
    2       |     35-36      |    16 Bits Per Sample
    4       |     37-40      |    "data" “data” chunk header. Marks the beginning of the data section
    4       |     41-44      |    file size (data) Size of data section
    
Note: Calculate number of samples in a second.
Note: Signature follows proper sin, wav pattern or distance of change from last sample to the next
```
Methodology
```
Signature Duration: data_chunk_size_bytes / byte_rate
byte_rate: Number of bytes for 1 second of audio

silence is classified as a section of audio having a max audio value of 4 and a min audio value of -4
within a given section of time in the signature. If there are no audio values that are greater or lesser
than these values then that section of defined audio will be excluded from the new wave file.

How sections of audio are defined:
Values:
    - 1: 1 or 1/1seconds of audio
    - 2: 0.5 or 1/2 seconds of audio
    - 4: 0.25 or 1/4th seconds of audio
    - 6: 0.16 or 1/6th seconds of audio
    - 10: 0.10 or 1/10th seconds of audio

Assuming:
    Signature Duration = 1800.5
    Sample Rate = 48000
    Byte Rate = 192000 = 1 second of audio
    
    Section value of 4
    Number of sections = Signature Duration * Section Value
    ie. (1800.5 * 4) i32 non-float value = 7202
    Max Bytes Window = Byte Rate / Section Value
    ie 192000 / 4 = 48000
    
    So for every 0.25 seconds of audio which is equal to 48000 bytes of signature data
    will be analyed to determine if that section contains only silence according to current thresholds
    bound
```