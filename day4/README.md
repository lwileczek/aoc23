# Day 4
## Part 1
I was pretty happy with how I didn't do any conversion from string to number nor call the split function. 
Because of that, I thought it would be pretty efficient having to do fewer allocation.
Made some assumptions about the format of the file which would make it difficult to use this elsewhere


## Part 2
I thought the question made it pretty obvious this is a recurson problem.
When I ran my solution I was right on the first try but it was so terribly slow I know I wrote some bad code.
I'll have to see how others did it to make the part 2 fast.
### Improved
Alright, so the key is to realize that the scores are just rolling upward so you can just pass the values
to the next loop/spot in the array.
No recurssion is required as since we can just put everything in a simple array.
Most importantly, with this method, we don't parse a line more than once.
