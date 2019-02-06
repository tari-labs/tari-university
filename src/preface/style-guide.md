# Style Guide

- [Purpose](#purpose)
- [Standards for Content](#standards-for-content)
  - [Spelling](#spelling)
  - [Units of Measure](#units-of-measure)
  - [Date and Time](#date-and-time)
  - [Abbreviations](#abbreviations)
  - [Spacing](#spacing)
  - [Decimals and Numbers](#decimals-and-numbers)
  - [Quotation Marks](#quotation-marks)
  - [List Types](#list-types)
  - [List Punctuation](#list-punctuation)
  - [Cross-referencing](#cross-referencing)
  - [Case Formatting](#case-formatting)
- [Standards for Layout](#standards-for-layout)
  - [Proposed Layout](#proposed-layout)
  - [Line Widths](#line-widths)
  - [Bulleted List of Contents](#bulleted-list-of-contents)
  - [Headings](#headings)
  - [Figures and Tables](#figures-and-tables)
  - [Equations](#equations)
  - [Referencing of Source Material](#referencing-of-source-material)
    - [Referencing Standard](#referencing-standard)
    - [Markdown Links](#markdown-links)
  - [List of Contributors](#list-of-contributors)
- [References](#references)
- [Contributors](#contributors)



## Purpose

The purpose of this Style Guide is to provide contributors to the Tari Labs University (TLU) reports with standards for content and layout. The intention is to improve communication and provide a high-quality learning resource for users. Use of this Style Guide can assist in ensuring consistency in the content and layout of TLU reports.

TLU content is created in [Markdown](https://www.markdownguide.org/) format and is rendered using 
[mdBook](https://github.com/rust-lang-nursery/mdBook).



## Standards for Content



### Spelling

Use the United States (US) spelling standard. The applicable dictionary is Merriam-Webster Online [[1]]. 



### Units of Measure

Use the internationally agreed ISO standards [[3]]  for expressing units of measure.

*Example*

min = minute, s = second, h = hour, g = gram. 



### Date and Time

- Date format: yyyy-mm-dd (year-month-date).
- Date format when written in text: "The document was submitted for approval on 10&nbsp;March&nbsp;2019".
- Time format (international): 11:00; 15:00.



### Abbreviations 

- If it is necessary to use abbreviations in a report, write the abbreviation out in full at its first occurrence in the text, followed by the abbreviation in brackets. Thereafter, use the abbreviation only.

  *Example*

  Tari Labs University (TLU), graphical user interface (GUI).

- Abbreviations of units should be consistent and not changed in the plural.

  *Example*

  10&nbsp;h and not 10&nbsp;hrs; 5&nbsp;min and not 5&nbsp;mins.



### Spacing

- Leave a space between numbers and units of measure.

  *Example*

  5&nbsp;min, 15&nbsp;s, 120&nbsp;°C (but 100°, 25%).

- Use a non-breaking space (`&nbsp;`) to prevent numbers and units from being split over two lines.

  *Example*

  ```Text
  On Tuesday, we measured a temperature of 34&nbsp;°C at sea level, i.e. 0&nbsp;m altitude, 
  while on the same day we measured 22&nbsp;°C on top of the mountain, which is 1&nbsp;500&nbsp;m high.
  ```

  Inserting the non-breaking spaces as shown in the example will result in the following (*play with the browser window to see the result*):

  On Tuesday, we measured a temperature of 34&nbsp;°C at sea level, i.e. 0
  &nbsp;m altitude, while on the same day we measured 22&nbsp;°C on top of the mountain, which is 1&nbsp;500&nbsp;m high.

- Indicate clearly to which unit a number belongs:

  *Incorrect example*

  11 x 11 x 11 mm

  *Correct example*

  11&nbsp;mm x 11&nbsp;mm x 11&nbsp;mm

- Use 'to' rather than a dash to indicate a range of values:

  *Incorrect example*
  1 - 10&nbsp;cm&nbsp;

  *Correct example*

  1&nbsp;cm to 10&nbsp;cm

- Use a non-breaking space (`&nbsp;`) space to indicate thousands.

  *Example*

  1&nbsp;000, 20&nbsp;000&nbsp;000, 250&nbsp;000.

Mathematical operators should usually be wrapped inside [equation tags](#equations). In plain text, leave a space on either side of signs such as + (plus), - (minus), = (equal to), > (greater than) and < (less than).



### Decimals and Numbers

- Use the decimal point and not the decimal comma.
- Write out numbers from one to nine in full in text; use Arabic numerals for 10 onwards.



### Quotation Marks

As per American style:

- Use double quotation marks for a first quotation and single quotation marks for a quotation within a quotation.
- Place commas and full stops inside the quotation marks.  



### List Types

TLU uses unordered lists (refer to the first example under [List Punctuation](#list-punctuation) and ordered lists (refer to the second example under [List Punctuation](#list-punctuation).



### List Punctuation

Where a list is a continuation of the preceding text, which is followed by a colon, use a semicolon between each bullet point, and end the list with a full stop.

*Example*

Their primary motivations for selecting a static emission rate are:

- there will be no upper limit on the amount of coins that can be created;
- the percentage of newly created coins compared to the total coins in circulation will tend toward zero;
- it will mitigate the effect of orphaned and lost coins;
- it will encourage spending rather than holding of coins.

Where a list contains complete sentences, each item in the list is followed by a full stop.

*Example*

According to the proposed solution, one of three conditions will be true to the SPV client when using erasure codes:

1. The entire extended data is available, the erasure code is constructed correctly, and the block is valid.
2. The entire extended data is available, the erasure code is constructed correctly, but the block is invalid.
3. The entire extended data is available, but the erasure code is constructed incorrectly.

Where a list is not a sentence and does not complete a preceding part of a sentence, use no punctuation.

*Example*

Refer to the list of contents at the start of this Style Guide.



### Cross-referencing

- Insert cross-references between the referenced information in the text and the list of references.
- Text references appear in square brackets in the text and are listed under "References" at the end of each chapter in the order in which they appear in the text.
- If a text reference appears at the end of a paragraph, it appears after the full stop at the end of the paragraph.
- Please be specific when referring to figures, tables and sections of text. For clarity, if using figure and table numbering, avoid referring to "below" or "above". Rather give a specific figure or table number. In the case of text references, include a link. For more information, please refer to the [Markdown Links](#markdown-links) section in this Style Guide.


### Case Formatting

???


## Standards for Layout

### Proposed Layout

This section gives the proposed layout for TLU reports. The following headings are provided as a guide to heading levels and content.  

- **Title (as heading level 1)**
   Contents List (as embedded links).
   - **Introduction/Purpose/Background/Overview (as heading level 2)**
      This section explains the aim of the report and prepares the reader for the content.

   - **Other headings as appropriate (as heading level 2 and lower)**
      Structure the body of your report by using headings and subheadings. Ordering these headings logically helps you to present your information effectively. Headings make it easier for readers to find specific information.
      - **Numbered Lists:** Use numbered lists when the order of the items in the list is important, such as in procedures.

      - **Bulleted Lists:** Use bulleted lists when the order of the items in the list is not important.

   - **Conclusions, Observations, Recommendations (as heading level 2)**
      The conclusion complements the purpose of the report. It concisely summarizes the findings of the report and gives a future strategy, if required.

   - **References (as heading level 2)**
      References acknowledge the work of others and help readers to find sources. Refer to [Referencing of Source Material](#referencing-of-source-material).

   - **Appendices (as heading level 2)**
      Appendices contains supplementary information that supports the main report. 
      - **Appendix A: Name (as heading level 3)**
        Rather than inserting an entire supporting document into an appendix, provide a text reference and list the reference in the references section.
      - **Appendix B: Name  (as heading level 3)**
        If figure and table numbers are used in the report, the figure and table numbering in the appendices follows on from the figure and table numbers used in the report.

   - **Contributors (as heading level 2)**
      Refer to [List of Contributors](#list-of-contributors).



### Bulleted List of Contents

Every chapter in a TLU report starts with a bulleted list of all the headings in that chapter (with embedded links), for quick reference and consistency. 

*Example*

Refer to the contents listed at the start of this Style Guide. The heading "Contents" is not inserted before this list. 



### Headings

- No numbering is used in the text panel. Numbering appears in the panel to the left of the text panel.
- For consistency, upper and lower-case (title case) letters are used for headings at all levels.

  

### Figures and Tables

The use of captions, as well as figure and table numbering, is optional. If you choose to use numbering and captions, these guidelines will help to promote consistency in TLU layout:

- Number figures and tables in each section sequentially, with the table caption above the table and the figure caption below the figure.
- Type figure and table captions in upper and lower-case (title case).
- Type Figure x: or Table X: before the caption, as applicable.
- Center figures and tables on the page.
- Place figures and tables as soon as possible after they are first referred to in the text. The text reference, if figure and table numbering is not used, would then be "the following figure..." or "the following table...." This helps to avoid confusion.



### Equations

mdBook has optional support for math [equations](https://github.com/rust-lang-nursery/mdBook/blob/master/book-example/src/format/mathjax.md) through MathJax. In addition to the delimiters `\[` and `\[`, TLU also supports delimiters `$` and `$$`.



*Examples*

Example of an inline equation: $ h \in \mathbb G $ 

Example of a display equation:
$$
 \mathbb s = \prod _{i=0}^n s(i)
$$



### Referencing of Source Material

#### Referencing Standard

TLU uses the IEEE standard [[2]] as a guide for referencing publications.

List references in the following order, as applicable:

1. Author(s) initials or first name and surname (note punctuation).

2. Title of the report, between double quotation marks. If it is an online report, state this in square brackets as shown in the following example.

3. Title of journal, in italics.

4. Publication information (volume, number, etc.).
5. Page range (if applicable).
6. URL address if an online publication. Provide this information as shown in the following example: "Available:..."
7. Date you accessed the article if it is an online publication (yyyy-mm-dd), as shown in the following example.

*Example* 

\[1\] M. Abe, M. Ohkubo and K. Suzuki, "1-out-of-n Signatures from a Variety of Keys" [online].
Available: https://www.iacr.org/cryptodb/archive/2002/ASIACRYPT/50/50.pdf. Date accessed: 2018-12-18.

Please note the use of punctuation and full stops in the example. 



#### Markdown Links

There are two types of Markdown links: **inline links** and **reference links**.

The **inline link** under the [Equations](#Equations) heading was created as follows:

1. Insert identifying link text within a set of square brackets (see following example).
2. Create an inline link by placing a set of parentheses (round brackets) immediately after the closing square bracket of the link text (see following example).
2. Insert the relevant URL link inside the parentheses (round brackets) (see following example).

mdBook has optional support for math [equations](https://github.com/rust-lang-nursery/mdBook/blob/master/book-example/src/format/mathjax.md) through MathJax.

*Example*

A **reference link** has two parts. The first part of a reference link has two sets of square brackets. Inside the inner (second) set of square brackets, insert a label to identify the link.

*Example* 

Under the heading [Spelling](#spelling), the text reference is "The applicable dictionary is Merriam-Webster Online [[1]]". Note the double square brackets, and the label 1.

The second part of a reference link is inserted under the heading [References](#references), and appears as follows:

[[1]] Merriam-Webster Online Dictionary [online]. 
Available: https://www.merriam-webster.com/. Date accessed: 2008-02-01.

[1]: https://www.merriam-webster.com
"Merriam-Webster Online Dictionary"

Where the full online reference is inserted after `[[1]]`; and the pop-up text link (which can be seen by hovering your cursor over the text reference in [Spelling](#spelling) is inserted after `[1]:`. 

For assistance with the layout of references, refer to [Referencing Standard](#referencing-standard).



### List of Contributors

The contributors are listed in a bulleted list via their github account URLs. The author is listed first, followed by any reviewers or people who contributed via pull requests. Refer to [Contributors](#contributors) for an example.


## References

[[1]] Merriam-Webster Online Dictionary [online]. 
Available: https://www.merriam-webster.com/. Date accessed: 2019-02-01.

[1]: https://www.merriam-webster.com
"Merriam-Webster Online Dictionary"

[[2]] Citing and Referencing: IEEE [online].
Available: https://guides.lib.monash.edu/citing-referencing/ieee. Date&nbsp;accessed:&nbsp;2019-02-01.

[2]: https://guides.lib.monash.edu/citing-referencing/ieee
"Citing and Referencing: IEEE"

[[3]] A. Thompson and B. N. Taylor, " Guide for the Use of the International System of Units (SI)", (1995) – NIST Special Publication 811 - 2008 Edition [online]. Available: https://physics.nist.gov/cuu/pdf/sp811.pdf. Date accessed: 2019-02-04.

[3]: https://physics.nist.gov/cuu/pdf/sp811.pdf
"Guide for the Use of the International System of Units (SI)"



## Appendices

### Appendix A: Lower Case Words used in Title Case Formatting

???



## Contributors

- https://github.com/anselld
- https://github.com/hansieodendaal
