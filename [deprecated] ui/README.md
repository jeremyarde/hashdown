# dioxus-template

> a template for starting a dioxus project to be used with [dioxus-cli](https://github.com/DioxusLabs/cli)

## Usage

#### use `dioxus-cli` init the template:

```
dioxus init hello-dioxus
```

or you can choose the template, for this tempalte:

```
dioxus init hello-dioxus --template=gh:dioxuslabs/dioxus-template
```

#### Start a `dev-server` for the project:

```
cd ./hello-dioxus
dioxus serve --hot-reload
```

or package this project:

```
dioxus build --release
```

## Project Structure

```
.project
- public # save the assets you want include in your project.
- src # put your code
- - utils # save some public function
- - components # save some custom components
```



## TODO

### Components

1. Editor
2. Question display
3. Publish button
4. Credits
5. instructions



## Example inputs

6. question one
   1. option 1
   2. option 2
7. testing


- this is another
  - option 1 in other
- test2 question
  - this is great



npx tailwindcss -i ./input.css -o ./public/output.css --watch