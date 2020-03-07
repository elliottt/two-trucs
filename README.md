
# two-trucs

> So, the French phrase trucs à faire means "things to do". J'ai des trucs à
> faire, "I have things to do." truc means 'thingamajig', basically.
>
> Anyway, you should call your tool `two-trucs`.

`two-trucs` is a tool for managing a grouped todo list written in markdown. For
example, if you have the following markdown:

```markdown
- [x] Task 1
- [x] Task 2
- [ ] Task 3
- [x] Add bugs
- [ ] Fix bugs
```

and you run `two-trucs` on it, it will re-order the list to the following:

```markdown
- [ ] Task 3
- [ ] Fix bugs
- [x] Task 1
- [x] Task 2
- [x] Add bugs
```

If you manage these lists by updating them daily, and want to archive the tasks
that were completed in the previous day, you can run `two-trucs` with the `-n`
flag, instructing it that you are beginning a new day. Given this markdown file:

```markdown
# Yesterday

## Project
- [x] Finish that thing
- [ ] Do that other thing
- [ ] Add bugs
```

Running `two-trucs -n` on the file will give you:

```markdown
# Today

## Project
- [ ] Do that other thing
- [ ] Add bugs

# Yesterday

## Project
- [x] Finish that thing
```

If you are starting a new day, you can alter the default heading of "Today" by
passing `-t "My alternate title"`.
