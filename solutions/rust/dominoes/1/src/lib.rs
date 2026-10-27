type Domino = (u8, u8);
type BitMask = u64;

pub fn chain(input: &[Domino]) -> Option<Vec<Domino>> {
    fn search(input: &[Domino], used: BitMask, chain: &mut Vec<Domino>) -> Option<Vec<Domino>> {
        if chain.len() == input.len() {
            let forms_closed_loop = chain
                .first()
                .zip(chain.last())
                .is_none_or(|(f, l)| f.0 == l.1);
            return forms_closed_loop.then(|| chain.clone());
        }
        let end = chain.last().map(|l| l.1);
        input
            .iter()
            .enumerate()
            // Select unused tiles.
            .filter(|&(i, _)| used & (1 << i) == 0)
            // Expand both orientations, keeping their index.
            .flat_map(|(i, &d)| [d, (d.1, d.0)].map(|cand| (i, cand)))
            // Select tiles that continue the chain.
            .filter(|&(_, cand)| end.is_none_or(|end| end == cand.0))
            // We try each candidate tile and orientation, and backtrack until a solution is found.
            .find_map(|(i, cand)| {
                chain.push(cand);
                let found = search(input, used | (1 << i), chain);
                chain.pop();
                found
            })
    }
    search(input, 0, &mut Vec::new())
}
