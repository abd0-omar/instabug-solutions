# Instabug Solutions
## SQL
### avg grades
```sql
SELECT students.name, AVG(grades.grade) AS average_grade
FROM grades
JOIN students ON grades.student_id = students.id
GROUP BY students.id
ORDER BY average_grade DESC, students.name ASC
LIMIT 100;
```
### courses enrolled
```sql
SELECT students.name AS name, courses.name AS course, grades.grade as grade
FROM grades
JOIN students ON sudents.id = grades.students_id
JOIN courses ON courses.id = grades.course_id
-- group by to remove the duplicates made by the JOIN clauses
GROUP BY students.name, course.name, grades.grade
ORDER BY students.name, course.name, grades.grade;
```
### most popular course
cte solution
```sql
WITH cte AS (
	SELECT courses.name, COUNT(*) as count
	FROM grades JOIN courses.id = grades.courses_id
	WHERE grades.grade >= 50
	GROUP BY grades.course_id
	ORDER BY count DESC
)
SELECT name FROM cte;
```
non cte solution
```sql
SELECT courses.name
FROM grades
JOIN courses ON courses.id = grades.courses_id
WHERE grades.grade >= 50
GROUP BY grades.course_id
ORDER BY COUNT(*) DESC;
```
### distinct bugs category
```sql
SELECT DISTINCT(category) AS category
FROM bugs;
```
### count em bugs
```sql
SHOW INDEXES FROM bugs;
```

```rust
let l = 0;
let r = 10049505;
while l <= r {
	let mid = l + (r - l) / 2;
	println!("SELECT created_at FROM bugs WHERE id >= {mid} LIMIT 1");
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	let input = input.trim().parse::<i32>().unwrap();
	if input == 1 { // go higher
		l = mid + 1;
	} else {
		r = mid - 1;
	}
}
```

```sql
SHOW CREATE TABLE bugs;
```

```sql
SELECT COUNT(*) as count
FROM bugs
WHERE id >= 4514546;
```
### Find the title
```sql
SHOW INDEXES FROM bugs
```

```sql
SELECT title
FROM bugs
-- category is a composite index on the most left hand side and you can search on more than one category using IN, eg. category IN ('bug', 'hamada')
WHERE token = 'token660' AND reported_at = '2020-08-30' AND category IN ('bug')
```
# References
[Instabug Backend Internship 2024 Task - Solution (youtube.com)](https://www.youtube.com/watch?v=xu17JBMnPZA&t=2648s)
