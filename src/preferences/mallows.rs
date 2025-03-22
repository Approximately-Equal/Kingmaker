use crate::core::*;
use rand::{distr::weighted::WeightedIndex, prelude::*, rngs::StdRng};

/// TODO...
#[derive(Debug, Clone)]
pub struct Mallows {
    pi_0: Vec<Id>,
    phi: f64,
}

impl Mallows {
    pub fn new(pi_0: Vec<Id>, phi: f64) -> Self {
        Self { pi_0, phi }
    }
}

impl Preference<Ordinal> for Mallows {
    fn draw(&self, candidate_pool: &[Candidate], rng: &mut StdRng) -> Ordinal {
        let distance = sample_kendall_tau_distance(self.pi_0.len(), self.phi, rng); // randomly sample a distance
        let permutation = sample_permutation_with_k_inversions(candidate_pool.len(), distance, rng); // randomly sample a permutation with the sampled distance

        let ranking: Vec<Id> = permutation.into_iter().map(|i| self.pi_0[i]).collect();

        Ordinal(ranking)
    }
}

/// Sample a Kendall tau distance using the Mallows model.
fn sample_kendall_tau_distance(n: usize, phi: f64, rng: &mut StdRng) -> usize {
    let max_inv = n * (n - 1) / 2; // maximum possible inversions

    // get proportions
    let weights: Vec<f64> = (0..=max_inv)
        .map(|inv| {
            let permutations = get_permutations(n, inv) as f64;
            permutations * (-phi * inv as f64).exp()
        })
        .collect();

    // sample from the distribution
    let dist = WeightedIndex::new(&weights).unwrap();
    dist.sample(rng)
}

/// Generate a uniform random permutation with exactly k inversions
fn sample_permutation_with_k_inversions(n: usize, k: usize, rng: &mut StdRng) -> Vec<usize> {
    let v_vector = sample_decomposotion_vector(n, k, rng);
    decomposition_to_permutation(&v_vector)
}

/// Uniformly generate a valid V(σ) decomposition vector with exactly k inversions
fn sample_decomposotion_vector(n: usize, k: usize, rng: &mut StdRng) -> Vec<usize> {
    let mut v = vec![0; n]; // decomposition vector
    let mut remaining_inv = k;

    for (i, item) in v.iter_mut().enumerate().take(n - 1) {
        if remaining_inv == 0 {
            break;
        }
        let slots = n - i - 1;
        let max_value = slots.min(remaining_inv);
        let min_value = remaining_inv - ((slots) * (slots - 1) / 2);
        let sampled_value = rng.random_range(min_value..=max_value);
        *item = sampled_value;
        remaining_inv -= sampled_value;
    }

    v
}

/// Convert V(σ) to a permutation σ
fn decomposition_to_permutation(v: &[usize]) -> Vec<usize> {
    let mut available = (0..v.len()).collect::<Vec<_>>();
    let mut perm: Vec<usize> = Vec::with_capacity(v.len());

    for sigma_i in v {
        let push = available.remove(*sigma_i);
        perm.push(push);
    }

    perm
}

/// Compute the number of permutations with exactly k inversions
fn get_permutations(n: usize, k: usize) -> usize {
    if n == 0 {
        return 0;
    };
    if k == 0 {
        return 1;
    };
    (0..=k.min(n - 1)).fold(0, |acc, i| acc + get_permutations(n - 1, k - i))
}
