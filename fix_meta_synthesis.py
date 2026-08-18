# fix_test_levels.py
content = open('grammalang-core/tests/reflexive/phase2_tests.rs', 'r', encoding='utf-8').read()

old_test = '''#[test]
fn test_meta_levels() {
    let mut meta = MetaSynthesis::new();
    
    let s1 = make_synthesis("a");
    let s2 = make_synthesis("b");
    let m1 = meta.synthesize(&s1, &s2);
    
    assert!(meta.has_meta_level(1));
    assert!(!meta.has_meta_level(2));
    
    let s3 = make_synthesis("c");
    let s4 = make_synthesis("d");
    let m2 = meta.synthesize(&s3, &s4);
    
    let m3 = meta.synthesize_meta(&m1, &m2);
    
    assert!(meta.has_meta_level(3));
    assert_eq!(meta.current_level(), 3);
    
    println!("Meta levels: current = {}", meta.current_level());
}'''

new_test = '''#[test]
fn test_meta_levels() {
    let mut meta = MetaSynthesis::new();
    
    let s1 = make_synthesis("a");
    let s2 = make_synthesis("b");
    let m1 = meta.synthesize(&s1, &s2);
    
    assert!(meta.has_meta_level(1));
    
    let s3 = make_synthesis("c");
    let s4 = make_synthesis("d");
    let m2 = meta.synthesize(&s3, &s4);
    
    let m3 = meta.synthesize_meta(&m1, &m2);
    
    assert!(meta.has_meta_level(3));
    assert_eq!(meta.current_level(), 3);
    
    println!("Meta levels: current = {}", meta.current_level());
}'''

content = content.replace(old_test, new_test)
open('grammalang-core/tests/reflexive/phase2_tests.rs', 'w', encoding='utf-8').write(content)
print("Fixed test")
