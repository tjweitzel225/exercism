use std::{collections::BTreeMap, iter::once};

#[derive(Default)]
struct Stats {
    mp: u16,
    w: u16,
    d: u16,
    l: u16,
    p: u16,
}

pub fn tally(match_results: &str) -> String {
    let mut table: BTreeMap<&str, Stats> = BTreeMap::new();
    for line in match_results.lines() {
        let mut split = line.split(';');
        let [team_a, team_b, result] = std::array::from_fn(|_| split.next().unwrap());
        match result {
            "win" => {
                let a_stats = table.entry(team_a).or_default();
                a_stats.w += 1;
                a_stats.mp += 1;
                a_stats.p += 3;
                let b_stats = table.entry(team_b).or_default();
                b_stats.l += 1;
                b_stats.mp += 1;
            }
            "draw" => {
                let a_stats = table.entry(team_a).or_default();
                a_stats.d += 1;
                a_stats.mp += 1;
                a_stats.p += 1;
                let b_stats = table.entry(team_b).or_default();
                b_stats.d += 1;
                b_stats.mp += 1;
                b_stats.p += 1;
            }
            "loss" => {
                let a_stats = table.entry(team_a).or_default();
                a_stats.l += 1;
                a_stats.mp += 1;
                let b_stats = table.entry(team_b).or_default();
                b_stats.w += 1;
                b_stats.mp += 1;
                b_stats.p += 3;
            }
            _ => unreachable!(),
        }
    }
    let mut v = Vec::from_iter(table);
    v.sort_by(|(team_a, stats_a), (team_b, stats_b)| {
        stats_b.p.cmp(&stats_a.p).then(team_a.cmp(team_b))
    });
    let rows = v.into_iter().map(|(team, stats)| {
        format!(
            "{:<30} |{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
            team, stats.mp, stats.w, stats.d, stats.l, stats.p
        )
    });
    once("Team                           | MP |  W |  D |  L |  P".to_string())
        .chain(rows)
        .collect::<Vec<_>>()
        .join("\n")
}
