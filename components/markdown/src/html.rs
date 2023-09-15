use config::RenderHooks;
use libs::ahash::HashMap;
use libs::once_cell::sync::Lazy;
use libs::pulldown_cmark as cmark;
use libs::pulldown_cmark::Event;

/// Default content that should be emitted when rendering each Markdown construct to HTML.
static DEFAULT_TAGS: Lazy<HashMap<MarkdownConstruct, TagContent>> = Lazy::new(|| {
    // # TODO: Use perfect hashing to optimize access times on the map
    // ! FIXME: Use `with_capacity`
    let mut map = HashMap::default();

    use ClosingTag::*;
    use MarkdownConstruct::*;

    map.insert(Heading1, ("h1", Mirror));
    map.insert(Heading2, ("h2", Mirror));
    map.insert(Heading3, ("h3", Mirror));
    map.insert(Heading4, ("h4", Mirror));
    map.insert(Heading5, ("h5", Mirror));
    map.insert(Heading6, ("h6", Mirror));

    map.insert(Paragraph, ("p", Mirror));
    map.insert(BlockQuote, ("blockquote", Mirror));
    map.insert(CodeBlock, ("pre", Mirror));

    map.insert(OrderedList, ("ol", Mirror));
    map.insert(UnorderedList, ("ul", Mirror));
    map.insert(Item, ("li", Mirror));

    map.insert(FootnoteDefinition, ("sup", Mirror));

    map.insert(Table, ("table", Mirror));
    map.insert(TableHead, ("thead", Mirror));
    map.insert(TableRow, ("tr", Mirror));
    map.insert(TableCell, ("td", Mirror));

    map.insert(Emphasis, ("em", Mirror));
    map.insert(Strong, ("strong", Mirror));
    map.insert(Strikethrough, ("strike", Mirror));
    map.insert(Link, ("a", Mirror));

    map.insert(Image, ("img", SelfClosing));

    map.insert(FootnoteReference, ("div", Mirror));

    map.insert(Rule, ("hr", Mirror));
    map.insert(TaskListMarker, ("input", Mirror));

    map
});

type TagContent = (&'static str, ClosingTag);

enum ClosingTag {
    Mirror,
    SelfClosing,
}

/// Represents each of the constructs we can emit from `pulldown_cmark`'s `Event`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MarkdownConstruct {
    // from `pulldown_cmark::Tag`
    Paragraph,
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    BlockQuote,
    CodeBlock,
    OrderedList,
    UnorderedList,
    Item,
    FootnoteDefinition,
    Table,
    TableHead,
    TableRow,
    TableCell,
    Emphasis,
    Strong,
    Strikethrough,
    Link,
    Image,

    // from `pulldown_cmark::Event`
    FootnoteReference,
    Rule,
    TaskListMarker,
}

pub fn push_html<'a, I>(output: &mut String, iter: I, render_hooks: &RenderHooks)
where
    I: Iterator<Item = Event<'a>>,
{
    cmark::html::push_html(output, iter)
}
