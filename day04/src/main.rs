use lib::read_all_lines_from_stdin;

// Space needs to be cleared before the last supplies can be unloaded from the
// ships, and so several Elves have been assigned the job of cleaning up
// sections of the camp. Every section has a unique ID number, and each Elf is
// assigned a range of section IDs.

struct SectionId(u32);

struct SectionAssignment {
    from: SectionId,
    to: SectionId,
}

// However, as some of the Elves compare their section assignments with each
// other, they've noticed that many of the assignments overlap. To try to
// quickly find overlaps and reduce duplicated effort, the Elves pair up and
// make a big list of the section assignments for each pair (your puzzle
// input).

struct AssignmentPair(SectionAssignment, SectionAssignment);

// For example, consider the following list of section assignment pairs:

// 2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8

impl From<&String> for AssignmentPair {
    fn from(value: &String) -> Self {
        todo!()
    }
}

// Some of the pairs have noticed that one of their assignments fully contains
// the other.

impl SectionAssignment {
    fn fully_contains(&self, other: &Self) -> bool {
        todo!()
    }
}

// In pairs where one assignment fully contains the other, one Elf in
// the pair would be exclusively cleaning sections their partner will already
// be cleaning, so these seem like the most in need of reconsideration.

impl AssignmentPair {
    fn needs_reconsideration(&self) -> bool {
        todo!()
    }
}

fn main() {
    let input: Vec<_> = read_all_lines_from_stdin().into_iter().collect();

    // In how many assignment pairs does one range fully contain the other?
}
