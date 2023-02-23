# Decode CLI

Quickly pull articles from https://decode.sh to read from the command line.

## Usage

The Decode CLI has a few different ways to pull articles.

### Search
The first way is to simply use the search command:
```
decode search "nodejs authentication"
```

this will return a list of articles for you to choose from.

### By Article Title
If you already know the title of the article you can simply run:
```
decode read article-title
```

This will open the article in a readers view.

### Using Tags
Looking for articles on a specific topic? Just use the tags flag:
```
decode search --tags nodejs authentication
```

This returns a list of articles similar to search but will return any article with at least one of the tags you passed.

### Search with Tags
You can also use the search command with tags and filter your results based on which articles have the requested tags:
```
decode search "authentication" --tags nodejs pocketbase svelte
```