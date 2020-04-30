# Technetium
Note: still in progress, check in soon

I've found a problem that needs a redesign of the stack frame system:

the program should look in its frame of definition for reference to locals. These frames might not exist at the time of the function call (because of functions as first class whatever). I must do a more traditional stack based execution context, with Arc's all around to local_indices.
