# timewarrior-aggregate

timewarrior-aggregate is an extension for timewarrior which helps to plan a day
and/or a week in advance and monitor the progress of the plan during the course
of the duration. If you are new to timewarrior, please find the [timewarrior
tutorial here](https://timewarrior.net/docs/tutorial/).

### Overview
The core idea of the extension is to be able to define “task groups'' which can
be allocated a fixed amount of time. Then, the extension can be run for either
a particular day or a particular week which will be described in the next
section.

For example, if we would like to spend 45 hours a week on the following:
| task group         | allocation |
|--------------------|------------|
| office project     | 15 hours   |
| office maintenance | 15 hours   |
| office reviews     |  5 hours    |
| office misc        |  5 hours    |
| personal learning  |  5 hours    |
| total              | 45 hours    |

We could break down the objective to something like:

1. Do the following on weekdays:
	| task group         | allocation |
	|--------------------|------------|
	| office project     | 2.5 hours  |
	| office maintenance | 2.5 hours  |
	| office reviews     |   1 hour   |
	| office misc        |   1 hour   |
	| personal learning  |   1 hour   |

2. Do the following on Saturday:
	| task group         | allocation |
	|--------------------|------------|
	| office project     | 2.5 hours  |

3. Do the following on Sunday:
	| task group         | allocation |
	|--------------------|------------|
	| office maintenance | 2.5 hours  |

The idea behind this extension is to be able to come up with such a set of task
groups and an allocation for a day and or a week upfront.  The objectives for
each day of the week could be fine tuned to achieve the objective of the week.

### Installation

The binary should be installed as a timewarrior extension at
~/.timewarrior/extensions/aggregate.

### Configuration

The usage of the extension is to help plan a day or a week. The task groups to
be tracked and their allocation for a day or a week is stored in separate JSON
files. All the files are rooted at ~/.timewarrior/aggregate/allocation. This is
followed by a file of the format `<year>/<month>/<day>.json`.

We get the below when we try to run the aggregate extension for the day initially.
```
$ timew aggregate :day
Unable to open the workgroups definition file for the day 2021-07-29 at /Users/ramakrishnan/.timewarrior/aggregate/allocation/2021/7/29.json.
Rerun the same command with SAMPLE=1 for a sample json file.
```

The message prints the path in which the configuration file is expected and
helps with a sample when the same command is repeated with environment variable
SAMPLE=1.
```
$ mkdir -p /Users/ramakrishnan/.timewarrior/aggregate/allocation/2021/7
$ SAMPLE=1 timew aggregate :day > /Users/ramakrishnan/.timewarrior/aggregate/allocation/2021/7/29.json
```

The sample configuration file generated looks like below:
```json
[
    {
        "tags": [
            "office",
            "project"
        ],
        "allocation": 3
    },
    {
        "tags": [
            "office",
            "maintenance"
        ],
        "allocation": 3
    },
    {
        "tags": [
            "office",
            "review"
        ],
        "allocation": 1
    }
]
```
Each entry contains information about a task group with tags defining the task
group and the amount of time allocated for it.

The generated sample could be modified to achieve file example objective for
the day described earlier.
```json
[
    {
        "tags": [
            "office",
            "project"
        ],
        "allocation": 2.5
    },
    {
        "tags": [
            "office",
            "misc"
        ],
        "allocation": 1
    },
    {
        "tags": [
            "office",
            "maintenance"
        ],
        "allocation": 2.5
    },
    {
        "tags": [
            "office",
            "review"
        ],
        "allocation": 1
    },
    {
        "tags": [
            "personal",
            "learning"
        ],
        "allocation": 1
    }
]
```

Similary running the aggregate extension for the week initially gives a message
like below:
```
$ timew aggregate :week
Unable to open the workgroups definition file for the week starting on 2021-07-26 at /Users/ramakrishnan/.timewarrior/aggregate/allocation/2021/7/week-of-26.json.
Rerun the same command with SAMPLE=1 for a sample json file.
```

Note that the only difference between a weekly allocation and a daily
allocation is that the weekly allocation file convention followed is
'week-of-x.json' where x is the starting day of the week. Timewarrior follows
the convention of treating Monday as the start of the week and the aggregate
extension follows the same.

### Usage

Once the json configuration is written to the file, the aggregate extension may
be run during the course of the day to see how much time should be spent on
each task groups.

timewarrior could be started to track each task with the tags from task groups
and optionally some information about what we worked on.
```
$ timew start office project "Implement task 1 of the awesome feature"
$ timew start office maintenance "Triage the nasty bug BUGXXXX"
```

It is convenient to define shell aliases like below which will help prevent
typos.
```
$ alias top='timew start office project'
$ alias tor='timew start office review'
$ alias tomsc='timew start office misc'
$ alias tom='timew start office maintenance'
$ alias tpl='timew start personal learning'
$ top "Implement task 1 of the awesome feature"
$ tom "Triage the nasty bug BUGXXXX"
```

For example, the aggregate extension for my day at the time of writing
this document looks like below:
```
$ timew aggregate :day
| group                | spent           | allocated       | remaining
| office project       | 2 hrs 1 mins    | 2 hrs 0 mins    | 0 hrs -1 mins
| misc office          | 0 hrs 8 mins    | 0 hrs 15 mins   | 0 hrs 6 mins
| maintenance office   | 3 hrs 59 mins   | 4 hrs 0 mins    | 0 hrs 0 mins
| office review        | 1 hrs 3 mins    | 1 hrs 0 mins    | 0 hrs -3 mins
| learning personal    | 0 hrs 41 mins   | 1 hrs 15 mins   | 0 hrs 33 mins
| total                | 7 hrs 54 mins   | 8 hrs 30 mins   | 0 hrs 35 mins
```
Negative number indicates exhausting the allocated time.

The aggregate extension for the week at the time of writing this document looks
like below:
```
$ timew aggregate :week
| group                | spent           | allocated       | remaining
| office project       | 9 hrs 59 mins   | 17 hrs 30 mins  | 7 hrs 30 mins
| misc office          | 3 hrs 1 mins    | 7 hrs 0 mins    | 3 hrs 58 mins
| maintenance office   | 14 hrs 6 mins   | 13 hrs 0 mins   | -1 hrs -6 mins
| office review        | 3 hrs 31 mins   | 5 hrs 0 mins    | 1 hrs 28 mins
| learning personal    | 2 hrs 53 mins   | 10 hrs 30 mins  | 7 hrs 36 mins
| total                | 33 hrs 33 mins  | 53 hrs 0 mins   | 19 hrs 26 mins
```

The final status of a past day or a week can be viewed by using the date range
feature in timewarrior. Some examples are given below:
```
$ timew aggregate 2021-07-19 to 2021-07-26
$ timew aggregate 2021-07-21 to 2021-07-22
```
