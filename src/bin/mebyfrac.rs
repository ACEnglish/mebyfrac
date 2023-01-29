use std::env;
use bio::io::fasta;
use std::collections::HashMap;

fn reverse(w: &String) -> String {
    /* 
     * Reverse operator
     */ 
    return w.chars().rev().collect::<String>();
}

fn compliment(w: &String) -> String {
    /*
     * Compliment operator
     */
    let mut table: HashMap<char, &str> = HashMap::new();
    table.insert('A', "T");
    table.insert('T', "A");
    table.insert('G', "C");
    table.insert('C', "G");
        

    let mut s = String::with_capacity(w.len());
    for c in w.chars() {
        match table.get(&c) {
            Some(rep) => s.push_str(rep),
            None => s.push(c),
        }
    }
    return s;
}

fn generate_kmers(k: usize, bases: &[char], current: String) -> Vec<String> {
    if k == 0 {
        return vec![current];
    }

    let mut kmers = vec![];
    for b in bases {
        let mut new_kmer:String = current.clone();
        new_kmer.push(*b);
        kmers.extend(generate_kmers(k - 1, bases, new_kmer));
    }
    kmers
}

fn same_set(k1:&String, k2:&String) -> usize {
    /*
     * Returns the math_table column index+1 for k2 relative to k1
     * if k2 is part of the same partition of k1 else 0
     */
    if *k1 == *k2 {
        return 1;
    }
    let r = reverse(&k1);
    if r == *k2 {
        return 4;
    }
    
    if compliment(&k1) == *k2 {
        return 3;
    }

    if compliment(&r) == *k2 {
        return 2;
    }
    return 0
}

fn create_generator_kmers(kmers: Vec<String>) -> Vec<String>{
    /* 
     * Generates the partitions of a set of vectors
     * Returns a hash map of a (random) key which represents the partition
     * and value of all kmers in the partition
     */

    let mut g_kmers:Vec<String> = vec![];
    for val in kmers {
        if g_kmers.iter().all(|e| same_set(e, &val) == 0) {
            g_kmers.push(val);
        }
    }
    return g_kmers;
}

fn main() {
    // Arg parsing
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let k_float = args[1].parse::<f64>().unwrap();
    let k_usize = k_float as usize;
    let file_path = &args[2];
    
    // probably const?
    let letters = vec!['A', 'T', 'C', 'G'];
    let comp_letters = vec!['T', 'A', 'G', 'C'];

    let n = 4_u32.pow(k_usize as u32); // total number of kmers
    let kfeat = vec![0; n as usize]; // counts of each kmer
    // Turn kmer into the index.. this will need to be a method
    //let mb = make_pow_array(4, k_float) ; equivalent to mb = 4 ** np.arange(k-1, -1, -1)
    // digits = np.array([m_letters.index(_) for _ in kmer])
    // index = digits * mb
    // kfeat[index] += 1
    //let digits = Array::usize?::zeros(k);

    // what is digits' size.. its k
    let kmers = generate_kmers(k_usize, &letters, "".to_string());
    // using the kmer indexing, how can I quickly find the generator kmer?
    let n_partition = create_generator_kmers(kmers);
    dbg!("generators: {:?}", &n_partition);

    let mut table: HashMap<String, f64> = HashMap::new();
    for g in &n_partition {
        table.insert(g.clone(), 0.0);
        table.insert(compliment(&g), 0.0);
        if *g != reverse(&g) {
            let r = reverse(&g);
            table.insert(r.clone(), 0.0);
            table.insert(compliment(&r), 0.0);
        }
    }

    dbg!("Reading Fasta");
    let mut records = fasta::Reader::from_file(file_path).expect("Unable to open").records();
    let mut tot_kmers:f64 = 0.0;

    while let Some(Ok(record)) = records.next() {
        // For multiple kmer values, subtract the smallest k
        // And sub loop the let s = block?
        let m_len = (record.seq().len() - k_usize) as f64;

        tot_kmers += m_len;
        let mut pos = 0;
        while pos < m_len as usize {
            let s = match String::from_utf8(record.seq()[pos..pos + k_usize].to_vec()) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            if table.contains_key(&s) {
                *table.entry(s.to_owned()).or_default() += 1.0;
            }

            pos += 1;
        }
    }
    dbg!("table {:?}", &table);

    let mut g_count:f64 = 0.0;
    let mut rg_count:f64 = 0.0;
    let mut cg_count:f64 = 0.0;
    let mut rcg_count:f64 = 0.0;
    for g in &n_partition {
        let tot = match table.get(g) {
                        Some(v) => v,
                        None => &0.0
        };
        g_count += tot;
        println!("x\tg\t{}\t{}", g, tot);

        let tot = match table.get(&compliment(g)) {
                        Some(v) => v,
                        None => &0.0
                    };
        cg_count += tot;
        println!("x\tcg\t{}\t{}", g, tot);


        let r = reverse(g);

        if *g != r {
            let tot = match table.get(&r) {
                            Some(v) => v,
                            None => &0.0
            };
            rg_count += tot;
            println!("x\trg\t{}\t{}", g, tot);

            let tot = match table.get(&compliment(&r)) {
                            Some(v) => v,
                            None => &0.0
                        };
            rcg_count += tot;
            println!("x\tcrg\t{}\t{}", g, tot);
        }
    }
    let eq6 = (g_count + rg_count) / tot_kmers;
    let eq7 = (cg_count + rcg_count) / tot_kmers;

    // Should report this as a json, or at least give a header to the tsv
    println!("tot\t{}",  tot_kmers);
    println!("g\t{}",  g_count);
    println!("rg\t{}",  rg_count);
    println!("cg\t{}",  cg_count);
    println!("rcg\t{}",  rcg_count);
    println!("eq6\t{}", eq6);
    println!("eq7\t{}", eq7);
}

