use std::io;
use std::env;
use bio::io::fasta;
use std::collections::HashMap;
use dahl_bellnumber::bell;
use ndarray::Array2;
use std::convert::TryFrom;

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

fn partition_kmers(kmers: &Vec<String>) -> HashMap<String, Vec<String>>{
    /* 
     * Generates the partitions of a set of vectors
     * Returns a hash map of a (random) key which represents the partition
     * and value of all kmers in the partition
     */
    let mut table: HashMap<String, Vec<String>> = HashMap::new();
    for val in kmers {
        // is the reverse in the table? add it
        let r = reverse(&val);
        if table.contains_key(&r) {
            println!("partition on r {}", val);
            table.get_mut(&r).map(|v| v.push(val.clone()));
            continue;       
        }
        let c = compliment(&val);
        if table.contains_key(&c) {
            println!("partition on c {}", val);
            table.get_mut(&c).map(|v| v.push(val.clone()));
            continue
        }
        let rc = compliment(&r);
        if table.contains_key(&rc) {
            println!("partition on rc {}", val);
            table.get_mut(&rc).map(|v| v.push(val.clone()));
            continue
        }
        println!("new partition {}", val);
        table.entry(val.clone()).or_insert(Vec::new()).push(val.clone());
    }
    return table;
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

fn math_table_rows(k: usize) -> usize {
    /* 
     * Calculate the number of rows a math table will hold
     */
    let n_k:u32 = u32::try_from(k).unwrap();
    let val:u32 = 2_u32.pow(n_k-1) + 4_u32.pow(n_k-1);
    return usize::try_from(val).unwrap();
}

// This is the main function
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let k = args[1].parse::<usize>().unwrap();
    let file_path = &args[2];

    dbg!("finding partitions for", &k);

    let bases = vec!['A', 'T', 'C', 'G'];
    let kmers = generate_kmers(k, &bases, "".to_string());
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
    //let mut records = fasta::Reader::new(reader).records();
    let mut tot_kmers:f64 = 0.0;

    while let Some(Ok(record)) = records.next() {
        let m_len = (record.seq().len() - k) as f64;

        tot_kmers += m_len;
        let mut pos = 0;
        while pos < m_len as usize {
            let s = match String::from_utf8(record.seq()[pos..pos + k].to_vec()) {
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

        let tot = match table.get(&compliment(g)) {
                        Some(v) => v,
                        None => &0.0
                    };
        cg_count += tot;


        let r = reverse(g);

        if *g != r {
            let tot = match table.get(&r) {
                            Some(v) => v,
                            None => &0.0
            };
            rg_count += tot;

            let tot = match table.get(&compliment(&r)) {
                            Some(v) => v,
                            None => &0.0
                        };
            rcg_count += tot;
        }
    }
    let eq6 = (g_count + rg_count) / tot_kmers;
    let eq7 = (cg_count + rcg_count) / tot_kmers;

    println!("tot\t{}",  tot_kmers);
    println!("g\t{}",  g_count);
    println!("rg\t{}",  rg_count);
    println!("cg\t{}",  cg_count);
    println!("rcg\t{}",  rcg_count);
    println!("eq6\t{}", eq6);
    println!("eq7\t{}", eq7);
}

