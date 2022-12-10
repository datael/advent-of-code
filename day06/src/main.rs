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

struct MarkerIsNDifferentCharacters<const N: usize>;

impl<const N: usize> CommunicationSystem for MarkerIsNDifferentCharacters<N> {
    fn lock_onto_signal(signal: String) -> usize {
        for (i, window) in as_window::<N>(signal.as_str()).enumerate() {
            // signal is all lower-case ascii;
            // make a bit-field on their ascii value (shifted so 'a' is 0) and count ones:
            // if we have N ones, then we have N different characters
            let mut l: u32 = 0;
            for c in window.chars() {
                l |= 1 << (c as u32 - 'a' as u32);
            }
            if l.count_ones() as usize == N {
                return i + N;
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
        .map(MarkerIsNDifferentCharacters::<4>::lock_onto_signal)
        .for_each(|offset| println!("Packet marker puts offset at {}", offset));

    // Your device's communication system is correctly detecting packets, but
    // still isn't working. It looks like it also needs to look for messages.

    // A start-of-message marker is just like a start-of-packet marker, except it
    // consists of 14 distinct characters rather than 4.

    input
        .iter()
        .map(Into::into)
        .map(MarkerIsNDifferentCharacters::<14>::lock_onto_signal)
        .for_each(|offset| println!("Message marker puts offset at {}", offset));
}
