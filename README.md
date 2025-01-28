# random-task
a small utility to take in a line delimited task list and return a random task. it can build a nested list based on the indentation in the case of having different categories of tasks. the nesting can be arbitrarily deep and it will drill down to the very bottom to return a single value. the nesting can be useful for if you have a category of tasks that are significantly larger, but you don't want to do a disproportionate amount of those tasks. Say you have 80 house projects and 20 chores. if your list was flat, you would have a 4/5 chance for a house project and a 1/5 chance for a chore. if you were to nest them under their category, you would have a 1/2 chance for either a chore or a house project. then you would have a 1/80 chance for a specific house project if you rolled a house project or a 1/20 for a specific chore if you rolled a chore.

nesting a list of tasks e.g.

```txt
chores
    clean
        kitchen
        bathroom
    fold clothes
fun
    code
    chill
```

you can also specify a different delimiter if you want to indent your list with something other than tabs, but those are the default delimiter
