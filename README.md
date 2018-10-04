# Rustfmt Line Counting Example

`rustfmt --check` produces a diff output which uses the line numbers
from the *formatted* file rather than the *source* file.

This makes it difficult to use --check in a CI context, where
you might want to look the error in the build log and know where to
look in your file.


Example:

For the source file `main.rs`
```
 1 fn my_shorter_function(
 2     _a: String,
 3     _b: String,
 4     _c: String,
 5 ) {
 6     unimplemented!()
 7 }
 8
 9 fn my_function_with_a_very_long_name(a: &str, b: &str, c: &str, d: &str, e: &str, f: &str) {
10     println!("A: {} | B: {} | C: {} | D: {} | E: {} | F: {}", a, b, c, d, e, f);
11 }
12
13 fn main() {
14     my_function_with_a_very_long_name("a", "b", "c", "d", "e", "f");
```

```
» rustfmt --check src/main.rs
Diff in /home/bradfier/src/bugs/rustfmt-linecount/src/main.rs at line 1:
-fn my_shorter_function(
-    _a: String,
-    _b: String,
-    _c: String,
-) {
+fn my_shorter_function(_a: String, _b: String, _c: String) {
     unimplemented!()
 }
Diff in /home/bradfier/src/bugs/rustfmt-linecount/src/main.rs at line 5:
 fn my_function_with_a_very_long_name(a: &str, b: &str, c: &str, d: &str, e: &str, f: &str) {
-    println!("A: {} | B: {} | C: {} | D: {} | E: {} | F: {}", a, b, c, d, e, f);
+    println!(
+        "A: {} | B: {} | C: {} | D: {} | E: {} | F: {}",
+        a, b, c, d, e, f
+    );
 }
 fn main() {
```

The diff `at line 5` actually appears at line 9 in the source file.
