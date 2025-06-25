<div align="center">

```
                     ,----,                                         
                   ,/   .`|                                    ,--. 
  .--.--.        ,`   .'  :    ,---,          ,----..      ,--/  /| 
 /  /    '.    ;    ;     /   '  .' \        /   /   \  ,---,': / ' 
|  :  /`. /  .'___,/    ,'   /  ;    '.     |   :     : :   : '/ /  
;  |  |--`   |    :     |   :  :       \    .   |  ;. / |   '   ,   
|  :  ;_     ;    |.';  ;   :  |   /\   \   .   ; /--`  '   |  /    
 \  \    `.  `----'  |  |   |  :  ' ;.   :  ;   | ;     |   ;  ;    
  `----.   \     '   :  ;   |  |  ;/  \   \ |   : |     :   '   \   
  __ \  \  |     |   |  '   '  :  | \  \ ,' .   | '___  |   |    '  
 /  /`--'  /     '   :  |   |  |  '  '--'   '   ; : .'| '   : |.  \ 
'--'.     /      ;   |.'    |  :  :         '   | '/  : |   | '_\.' 
  `--'---'       '---'      |  | ,'         |   :    /  '   : |     
                            `--''            \   \ .'   ;   |,'     
                                              `---`     '---'       
```

</div>

<p align="center">
    <img src="https://github.com/connortbot/stack/actions/workflows/rust.yml/badge.svg" alt="build">
</p>

<p align="center">
<i>
Simple PR stacking.
</i>
</p>

`stack` is a tool that sits on top of native `git` for tracking and propagating changes up [**PR stacks**](https://www.stacking.dev/). It acts as an alternative to other stacking tools by focusing on minimizing changes made to your workflow.

> *(Other tools like `git-town` or `graphite` offer a much more mature and feature-rich toolset for PR stacking. Use `stack` if you want an easy and quick PR stacking solution.)*

# How do I use `stack`?
> Stacking parallelizes your development and code review workstreams, so you don't need to wait for your previous changes to be merged before building on top of them. - `stacking.dev`

If you work on a team, you're probably doing some form of PR stacking anyways! Here's a simple example with one stack.

Suppose you have a feature broken down into 3 branches:
1. `database-change`
2. `backend-change`
3. `frontend-change`

You can easily manage this with `stack`!

```bash
# First, initialize
stack init

# Create your stack
stack checkout -c feature

# Add your first branch
stack push database-change

# Add the next
stack push backend-change
```

Let's say that you need to update something on the bottom of your stack.

```bash 
# Propagate changes up the stack
stack rebase --onto-main
```
This pulls latest changes on `main`, and rebases everything like so:
```
main ----> database-change ----> backend-change
```

This becomes *especially useful* when you have a stack of several PRs, where propagating changes is very tedious.

Some other useful commands:
```bash
# Managing stacks
stack checkout -c stack_a # create
stack checkout stack_b # move to other stack
stack delete stack_to_delete
stack list # show stacks
stack status # show current stack

# Editing stacks
stack insert in-between-change --index 1
stack remove stupid-change --index 4
stack pop # removes last
stack shift # removes first
stack push
```

# Installation
For Mac:
```bash
curl -L -o stack https://github.com/connortbot/stack/releases/latest/download/stack

chmod +x stack
mv stack /usr/local/bin/
```

# Contributing
Feel free to [open an issue](https://github.com/connortbot/stack/issues/new) or a PR!

Bug fixes are very welcome! To keep `stack` minimal, the only large change planned for now is adding labels to branches in a stack.