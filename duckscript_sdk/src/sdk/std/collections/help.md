The collections module contains commands which enable to interact with different data models such as arrays, sets and maps.

* Arrays are simple ordered list of items
* Sets are unordered unique collection of items
* Maps are key/value (dictionary) structure where the keys are unique

Access to these data structures are done via handles.<br>
Handles are provided by the data structure creation command (such as: array, range, map, set) and are used in all
other commands to read/modify those data structures.<br>
Once done with a specific data structure, you must release it via release command to prevent any memory leaks.

