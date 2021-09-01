# A Map programming language (rather than an Array language)
APL, J, K etc. are all array languages, and as such the array is the core object they manipulate. Conceptually however a lot of data is more naturally represented by a map structure (e.g. Python dictionary).

For example, employee data, in APL you may have:
```
salaries ←  500 1000 2400
ages ← 22 25 23
```
and manipulations rely on these arrys orders, e.g. the first item of the salaries array and the first item of the ages arrays correspond to the first employee and so on.

I propose that it is much more natural to manipulate this data as a (an array of) map(s):
```
employee_data <- [ { 'salary' : 500, 'age' : 22 }, { 'salary' : 1000, 'age' : 25 }, { 'salary' : 2400, 'age' : 23 } ]
```

This language aims to allow for APL style manipulation of map data.

# Example: employee average values
The following program computes the average values of employee data (those familiar with APL will recognise the pattern):
```
employee_data <- [ { 'salary' : 500, 'age' : 22 }, { 'salary' : 1000, 'age' : 25 }, { 'salary' : 2400, 'age' : 23 } ]
averages <- (+/ employee_data) ÷ ⍴ employee_data
print averages
```
which gives output of:
```
PRINT Dictionary({"age": Numeric(Float(23.333333333333332)), "salary": Numeric(Float(1300.0))})
```

A short explanation of the above program is that on the left of the divide, performing a reduciton of '+' over the structures, to produce a total structure, then dividing this by the right hand side value which is the number of items in the employee_data array.

# Example: extract values from array of dictionaries
A common action is to run some computation on some specific fields of dictionaries in aggregate:
```
employee_data <- [ { 'salary' : 500, 'age' : 22 }, { 'salary' : 1000, 'age' : 25 }, { 'salary' : 2400, 'age' : 23 } ]
salaries <- employee_data.'salary'
average_salary <- (+/ salaries) ÷ ⍴ salaries
print average_salary
```
which gives output of:
```
PRINT Numeric(Float(1300.0))
```
where the important piece of code here is:
```
employee_data.'salary'
```
which creates an array of only the salaries [ 500, 1000, 2400 ] extracting just that component.

TODO: implement extraction of sub-structure a.['salary', 'age'] which can pull multi fields 
## Resources
- useful guide to rusts module system: http://www.sheshbabu.com/posts/rust-module-system/ 
- pest docs parsing j lang: https://pest.rs/book/examples/jlang.html