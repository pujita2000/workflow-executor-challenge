### Approach and Design Decisions

As someone who likes to understand the big picture before diving into the task details, I first chose to take a while to understand the assignment in depth. I got the opporunity to learn about Workflows beyong my basic SWE level understanding (it's always easy to push this towards the DevSecOps folks...). Curiously, despite being a Rust engineer for years, tokio is not a daily part of my toolset, so my appoach was understanding what the simplest, most non-intensive yet thorough way to use the crate would be for the challenge.

Before getting into the code, I restructured the repo so that the types were naturally sorted into their respective files. This makes the files less bloated, allows for unit tests to be easily referenced, and helps my brain (and I hope other's) make an easy mental model of the repo. I also made some comments, type changes, helpers, and tests off the bat to both solidify my understanding and give me peace of mind.

If you take a look at the remote Git repo, you'll see that, perhaps oddly, I had branches and PRs for this challenge. Though, the challenge was simple enough to not need such additions, I have learned that for the sake of organization and future success, it's often nice to have that structure off the bat. If I was to build off this work, I'd like to see my commit, PR, and feature history. Also, it makes it really easy to review your work and catch small typos, bugs, and odditities if you make PRs (this is not bind myself to claiming my work is free of all of that...).

In terms of the application itself, my work is sectioned to create as much isolation as possible. For instance, `Workflow::validate()` is purposefully kept small with helper functions doing the grunt work, and the same goes for `SimpleExecutor::execute()`. This allows for extremely detailed testing, clean, readable code, and sets us up to add more complexities without headache. I also chose to have `GraphState` early on because having an interim state wrapped in a Rust data type means safe and easily followable code.

### Challenges faced / tradeoffs

My main challenge was thoroughly understanding the task. I occassionally struggle to "just do it" and like knowing a task inside and out. Because I have not written a simple executor before, this meant taking the time to read through all the material, which is slow. However, I have lots of practice reading code for MR reviews as well as tickets and delivery requirements for customers, so it was not unfamiliar. Another challenge was that I was not sure the "best" approach with tokio. There was the question of simplicity and ease, vs setting up for scalability and speed. This is why in the branch `ps/phase3`, you can see a simpler approach to the problem which is not as robust as what is in main.

### Ideas for future improvements

An improvement would be executing multiple workflows at once. In theory it is essentially a similar approach of concurrent tasks (perhaps threads for more intensive jobs) executing multiple workflows at once, each with their own SimpleExecutor. A solid improvement would be cancellation / timeout support per node. For more complex tasks, timeouts could prevent hung tasks, and the ability to propagate cancellation to dependent nodes would prevent wasted resources/time and keep the workflow state most accurate. For fun, I wanted to add in a tool for the workflow executor outside of testing. At first I thought a cli tool, but what is the use case for a cli tool in this challenge? So then a UI might be nice to see the executor go off (nod at GitHub actions).

### Time spent

I spent about 7-8 hours on this task. In the effort to be honest, I probably spent more time on this challenge, but that involved me going into Rust rabbit holes when reading the task, learning about crates that I haven't used as well as concepts I just wanted to learn more about, and organizing/writing a lot. So actual challenge completion I'll say 7-8, and if I was to do it again, I could probably do it well in 2-3 hours.

### AI in my workflow

I mainly used AI tools as a PA. It was my rubber ducky, explained the concepts and requirements that I wanted more details on, and catch anything I missed in my understanding. I like using AI to read my work and help me catch bugs or identify opportunities to enhance my code (the Rust analyzer is a great tool for this too). I also used it to write or check my unit tests.
