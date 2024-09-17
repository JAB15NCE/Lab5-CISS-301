/*
Name: Jon Bennett
Date: 02/24/24
Class: CISS 301

Discription:
Runs the pre-determine sets and pairs given in the main funciton against the relation traits within 
the impl relation function through a bool to determine whether that set and pairs meet that trait requirement
if they do it will return true, if not then false. 
*/

use std::collections::HashSet;

struct Relation {
    relation: HashSet<(String, String)>,
}

//Group of fucntion put into a implmention of the function Relation. This is where each set and pair are ran through and checked if
//true or false again if the meet the funcitons requirements it will return true if not then false. 
impl Relation {
    fn is_reflexive(&self) -> bool {
        let elements: HashSet<&String> = self.relation.iter().map(|(x, _)| x).collect();
        elements.iter().all(|&x| self.relation.contains(&(x.clone(), x.clone())))
    }

    fn is_symmetric(&self) -> bool {
        self.relation.iter().all(|(x, y)| self.relation.contains(&(y.clone(), x.clone())))
    }

    fn is_transitive(&self) -> bool {
        self.relation.iter().all(|(x, y)| {
            self.relation.iter().filter(|&(u, _)| u == y).all(|(_, v)| self.relation.contains(&(x.clone(), v.clone())))
        })
    }

    fn is_antisymmetric(&self) -> bool {
        self.relation.iter().all(|(x, y)| x == y || !self.relation.contains(&(y.clone(), x.clone())))
    }

    fn is_equivalence(&self) -> bool {
        self.is_symmetric() && self.is_reflexive() && self.is_transitive()
    }
}

//This is the main function and maps each element within the pair to a string to then be mapped and stored within a vec
//to be ran against each function trait above. 
fn main() {
    
    // First Set and Pairs
    let _set1 = ['a', 'b', 'c', 'd', 'e'];
    let pairs1 = vec![
        ('a'.to_string(), 'b'.to_string()),
        ('a'.to_string(), 'e'.to_string()),
        ('b'.to_string(), 'c'.to_string()),
        ('c'.to_string(), 'd'.to_string()),
        ('d'.to_string(), 'f'.to_string()),
    ];

    let relation1 = Relation {
        relation: pairs1.iter().cloned().collect(),
    };

    println!("Set 1:{:?}", _set1);
    println!("pair:{:?}", pairs1);
    println!("Reflexive: {}", relation1.is_reflexive());
    println!("Symmetric: {}", relation1.is_symmetric());
    println!("Transitive: {}", relation1.is_transitive());
    println!("Antisymmetric: {}", relation1.is_antisymmetric());
    println!("Equivalence: {}", relation1.is_equivalence());

    // Second Set and Pairs
    let _set2 = [1, 3, 5, 7, 8, 12, 13];
    let pairs2 = vec![
        (1.to_string(), 8.to_string()), 
        (1.to_string(), 12.to_string()), 
        (3.to_string(), 8.to_string()), 
        (3.to_string(), 12.to_string()), 
        (5.to_string(), 8.to_string()), 
        (5.to_string(), 12.to_string()), 
        (7.to_string(), 8.to_string()), 
        (7.to_string(), 12.to_string()),
    ];

    let relation2 = Relation {
        relation: pairs2.iter().cloned().collect(),
    };

    println!("\nSet 2:{:?}", _set2);
    println!("pairS:{:?}", pairs2);
    println!("Reflexive: {}", relation2.is_reflexive());
    println!("Symmetric: {}", relation2.is_symmetric());
    println!("Transitive: {}", relation2.is_transitive());
    println!("Antisymmetric: {}", relation2.is_antisymmetric());
    println!("Equivalence: {}", relation2.is_equivalence());

    // Third Set and Pairs
    let _set3 = ["()", "!", "*", "+", "<<", "<", "&"];
    let pairs3 = vec![
        ("+".to_string(), "*".to_string()), 
        ("*".to_string(), "()".to_string()), 
        ("*".to_string(), "!".to_string()), 
        ("+".to_string(), "<<".to_string()), 
        ("<<".to_string(), "<".to_string()), 
        ("<<".to_string(), "&".to_string()),
    ];

    let relation3 = Relation {
        relation: pairs3.iter().cloned().collect(),
    };

    println!("\nSet 3:{:?}", _set3);
    println!("pairs:{:?}", pairs3);
    println!("Reflexive: {}", relation3.is_reflexive());
    println!("Symmetric: {}", relation3.is_symmetric());
    println!("Transitive: {}", relation3.is_transitive());
    println!("Antisymmetric: {}", relation3.is_antisymmetric());
    println!("Equivalence: {}", relation3.is_equivalence());

    // Fourth Set and Pairs
    let _set4 = ['m', 'd', 'b', 's', 'g', 'f'];
    let pairs4 = vec![
        ('m'.to_string(), 'b'.to_string()), 
        ('m'.to_string(), 's'.to_string()), 
        ('d'.to_string(), 'b'.to_string()), 
        ('d'.to_string(), 's'.to_string()), 
        ('g'.to_string(), 'm'.to_string()), 
        ('f'.to_string(), 'm'.to_string()),
    ];

    let relation4 = Relation {
        relation: pairs4.iter().cloned().collect(),
    };

    println!("\nSet 4:{:?}", _set4);
    println!("pairs:{:?}", pairs4);
    println!("Reflexive: {}", relation4.is_reflexive());
    println!("Symmetric: {}", relation4.is_symmetric());
    println!("Transitive: {}", relation4.is_transitive());
    println!("Antisymmetric: {}", relation4.is_antisymmetric());
    println!("Equivalence: {}", relation4.is_equivalence());
    
    // My first set and pairs
    let _myset1 = ['a', 'b', 'c', 'd', 'j', 'o', 'n'];
    let mypair5 = vec![
        ('a'.to_string(), 'a'.to_string()), 
        ('b'.to_string(), 'b'.to_string()),
        ('c'.to_string(), 'c'.to_string()), 
        ('d'.to_string(), 'd'.to_string()), 
        ('j'.to_string(), 'j'.to_string()), 
        ('o'.to_string(), 'o'.to_string()),
        ('n'.to_string(), 'n'.to_string()),
        ];

    let relation5 = Relation {
        relation: mypair5.iter().cloned().collect(),
    };

    println!("\nMySet 1, set 5:{:?}", _myset1);
    println!("pairs:{:?}", mypair5);
    println!("Reflexive: {}", relation5.is_reflexive());
    println!("Symmetric: {}", relation5.is_symmetric());
    println!("Transitive: {}", relation5.is_transitive());
    println!("Antisymmetric: {}", relation5.is_antisymmetric());
    println!("Equivalence: {}", relation5.is_equivalence());

    //My second set and pair. 
    let _myset2 = ["cat", "dog", "bird", "frank", "jon", "bill"];
    let mypair6 = vec![
        ("cat".to_string(), "cat".to_string()),  
        ("dog".to_string(), "dog".to_string()),  
        ("bird".to_string(), "bird".to_string()),
        ("frank".to_string(), "jon".to_string()),
        ("jon".to_string(), "frank".to_string()),
        ("bill".to_string(), "bill".to_string()),
        ("jon".to_string(), "jon".to_string()),
        ("frank".to_string(), "frank".to_string()), 
    ];

    let relation6 = Relation {
        relation: mypair6.iter().cloned().collect(),
    };

    println!("\nMySet 2, set 6:{:?}", _myset2);
    println!("pairs:{:?}", mypair6);
    println!("Reflexive: {}", relation6.is_reflexive());
    println!("Symmetric: {}", relation6.is_symmetric());
    println!("Transitive: {}", relation6.is_transitive());
    println!("Antisymmetric: {}", relation6.is_antisymmetric());
    println!("Equivalence: {}", relation6.is_equivalence());
}
