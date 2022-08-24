+++
author = "Alex Bilson"
date = "2021-06-05"
lastmod = "2021-12-30 11:23:21"
epistemic = "plant"
tags = ["note-taking","design"]
+++
There's a magic in Andy Matuschak's [Digital Garden](https://notes.andymatuschak.org/). The glory of it's deeply-linked organization is greatly aided by his custom viewport. Navigation happens like this:

To browse the contents of a linked note, hover over for a snapshot. To review the full note, click the link and it'll appear to the right as though it's been added to a stack of notecards. One can follow links down a thought-path, viewing the last note and the present note side-by-side, or peruse several nodes off the same note without ever using the browser's Back button.

Notes which refer to the open note are listed with short excerpts at the bottom of the note. These links broaden one's view of Andy's content while the embedded links navigate to more specific or nearby notes.

# What's Missing Today

I would like to implement a similar system, though I don't have the bandwidth to build my own viewport. Why would I want this? To have a single, distributed network of growing thoughts and ideas with permanence for myself and to share with posterity.

I have a sense of permanence in what exists today with plaintext files and a single place to record and peruse thoughts on this site. However, the distributed network and its navigation remains elusive.

Though [tags are an ineffective association structure](https://notes.andymatuschak.org/z3MzhvmesiD2htMaEFQJif7gJgyaHAQvKH49Z), I've begun a structure for my notes as a [network of interrelated thoughts](https://alexbilson.dev/network/). I have a problem with a mixture of concepts and content types, but I do find that my diagram supplies newcomers a navigable overview of my site's content. The overview serves a distinct purpose from association. Association allows _me_ to wander the corridors of my thoughts, while tag networks hand a visitor the structural map of all thoughts I've documented online and how they conceptually relate.

{{< notice name="Quiche" src="https://notes.andymatuschak.org/z5efx2iNLSB8antyDHfU74Xk3x7voSXk9tuec" >}}
A tag network would help newcomers on Andy's site too. I would never realize from the starting page that Andy has a note about quiche.
{{< /notice >}}

How to achieve this with [Hugo](https://gohugo.io/)?

# Garden Approaches

Maggie Appleton tends a [repository of digital gardening tools and users](https://github.com/MaggieAppleton/digital-gardeners/). This is how I discovered Andy's system and many others. To my delight, bi-directional hyperlink implementations are available even for static sites! To my chagrin, none that aren't baked into tools I don't use.

The primary requirement to convert my existing static content into a network is the implementation of bi-directional hyperlinks, so let's start there.

## Approach #1 - Static Build Step

A primary goal for my build process is to have one set of repeatable steps that always produce the same result. With this priority I hacked together my tag-parser Golang module. This module parses tags from every content file and generates a JSON file that characterizes the relationships between tags and files. It's what runs my [network](https://alexbilson.dev/network/) page.

If I followed the same pattern I could parse the content files for internal hyperlinks, then generate backlink lists per URL. But the most likely place to store those backlinks would be the source file itself. That would require my build process to both use _and_ modify my content files - danger zone. I might insert them into the HTML output instead, but Hugo doesn't supply any hook to enter custom HTML so I'll have to enter them after the file is fully formed - yuck.

{{< notice name="Vim Note-taking" src="https://www.edwinwenink.xyz/posts/42-vim_notetaking/" >}}
Despite the danger zone nature of editing the original content files as part of my build, I do like that I might use the backlinks when navigating my content with my favorite editor, Vim.
{{< /notice >}}

## Approach #2 - Sqlite Database

If I reconsider content as the place to store backlinks, then a sqlite database is my next option. The advantage in changing the content is that a browser doesn't need to look elsewhere to retrieve backlinks since they'll be part of the original HTML payload. With this route I'll need to write a service to pull backlinks into my posts, but that's not so different from what happens with comments today. I can still keep the build pattern to generate the sqlite file at least.

I would prefer sqlite because of its portability. It will be read-only, so the write limitations inherent in this storage type won't affect me.

## Approach #3 - WebMention

If I'm going to use a sqlite database instead of plaintext alone, I could utilize additional technologies to keep from building every backlink from scratch each time I add a file. Enter the WebMention.

Since I'd first heard of WebMention, I thought of it as a standard for intrasite communication. But there's no reason it might not be used for intersite communication.

When I publish new content, an automated WebMention POST could be sent with the filename and internal hyperlinks. My WebMention service will generate backlink records for each reference that doesn't exist. If I navigate to one of the referenced posts, a JavaScript event will query my WebMention service, include the new record, and generate a backlink to my fresh content.

## Approach #4 - By Hand

Who says I need to automate? Yes, it'll balloon out of control if I'm adding notes with multiple internal hyperlinks every day, but maybe I'm engineering a solution to a limited problem?

# Conclusion

(add the decision to use hugo shortcodes and handle this by hand for now)
