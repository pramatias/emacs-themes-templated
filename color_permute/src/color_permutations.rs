extern crate itertools;
use itertools::Itertools;


pub fn process_color_permutations<'a>(colors: &'a [&'a str]) -> Vec<Vec<&'a str>> {
    let permutations = permute_colors(colors); // Pass colors by reference

    // Generate unique permutations
    let unique_perms = unique_permutations(permutations);

    unique_perms
}

fn permute_colors<'a>(colors: &'a [&'a str]) -> Vec<Vec<&'a str>> {
    colors.iter().permutations(colors.len()).map(|p| p.into_iter().cloned().collect()).collect()
}

fn unique_permutations(mut permutations: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    let mut unique_permutations = Vec::new();

    while !permutations.is_empty() {
        let mut perm = permutations.pop().unwrap();

        // Find the index of the smallest color
        let min_index = perm
            .iter()
            .enumerate()
            .min_by_key(|&(_, color)| *color)
            .map(|(index, _)| index)
            .unwrap();

        // Shift the colors until the smallest one is at the beginning
        perm.rotate_left(min_index);

        let mut is_unique = true;

        for other_perm in &permutations {
            let mut other_perm = other_perm.clone();

            // Find the index of the smallest color in the other permutation
            let min_index = other_perm
                .iter()
                .enumerate()
                .min_by_key(|&(_, color)| *color)
                .map(|(index, _)| index)
                .unwrap();

            // Shift the colors until the smallest one is at the beginning
            other_perm.rotate_left(min_index);

            if perm == other_perm {
                is_unique = false;
                break;
            }
        }

        if is_unique {
            unique_permutations.push(perm);
        }
    }

    unique_permutations
}
