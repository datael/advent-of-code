use lib::read_all_lines_from_stdin;

// As you move through the dense undergrowth, one of the Elves gives you a
// handheld device. He says that it has many fancy features, but the most
// important one to set up right now is the communication system.

// To be able to communicate with the Elves, the device needs to lock on to
// their signal. The signal is a series of seemingly-random characters that
// the device receives one at a time.

trait CommunicationSystem {
    fn lock_onto_signal(signal: String) -> usize;
}

// To fix the communication system, you need to add a subroutine to the device
// that detects a start-of-packet marker in the datastream. In the protocol
// being used by the Elves, the start of a packet is indicated by a sequence
// of four characters that are all different.

struct PacketMarkerIsNDifferentCharacters;

impl CommunicationSystem for PacketMarkerIsNDifferentCharacters {
    fn lock_onto_signal(signal: String) -> usize {
        for (i, window) in as_window::<4>(signal.as_str()).enumerate() {
            // signal is all lower-case ascii;
            // make a bit-field on their ascii value and count ones
            // if we have 4 ones, then we have four different characters
            let mut l: u32 = 0;
            for c in window.chars() {
                l |= 1 << c as u32 - 'a' as u32;
            }
            if l.count_ones() == 4 {
                return i + 4;
            }
        }

        0
    }
}

fn as_window<const N: usize>(input: &'_ str) -> impl Iterator<Item = String> + '_ {
    input.char_indices().flat_map(move |(i, _)| {
        if i + N <= input.len() {
            Some(input[i..i + N].to_string())
        } else {
            None
        }
    })
}

fn main() {
    let input: Vec<_> = read_all_lines_from_stdin().into_iter().collect();

    input
        .iter()
        .map(Into::into)
        .map(PacketMarkerIsNDifferentCharacters::lock_onto_signal)
        .for_each(|offset| println!("Packet marker puts offset at {}", offset));

    // as_window::<4>("abcdefghi").for_each(|s| println!("{s}"));
}
