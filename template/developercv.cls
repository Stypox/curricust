%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Developer CV
% LaTeX Class
% Version 3.0 (07/11/23)
%
% Originally taken from
% https://www.overleaf.com/latex/templates/cv-developer/rdycxzvvnvcc
%
% Authors:
% Adapted by Stypox for use with Curricust
% Based on a template by Omar Roldan
% Based on a template by Jan Vorisek (jan@vorisek.me)
% Based on a template by Jan Küster (info@jankuester.com)
% Modified for LaTeX Templates by Vel (vel@LaTeXTemplates.com)
%
% License: MIT
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%----------------------------------------------------------------------------------------
%	CLASS CONFIGURATION
%----------------------------------------------------------------------------------------

\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{cvtemplate}[2023/10/12 Developer CV class v2.0]

\DeclareOption*{\PassOptionsToClass{\CurrentOption}{extarticle}} % Pass through any options to the base class
\ProcessOptions\relax % Process given options

\LoadClass{extarticle} % Load the base class

%----------------------------------------------------------------------------------------
%	PACKAGES AND OTHER DOCUMENT CONFIGURATIONS
%----------------------------------------------------------------------------------------

\setlength{\parindent}{0mm} % Suppress paragraph indentation

\usepackage[hidelinks]{hyperref} % Required for links but hide the default boxes around links

\pagestyle{empty} % No headers or footers

\usepackage{moresize} % Provides more font size commands (\HUGE and \ssmall)
\usepackage{enumitem}
%----------------------------------------------------------------------------------------
%	PAGE LAYOUT
%----------------------------------------------------------------------------------------

\usepackage{geometry} % Required for adjusting page dimensions and margins

\geometry{
	paper=a4paper, % Paper size, change to letterpaper for US letter size
	top=1.6cm, % Top margin
	bottom=1.6cm, % Bottom margin
	left=1.6cm, % Left margin
	right=1.6cm, % Right margin
	headheight=0.75cm, % Header height
	footskip=1cm, % Space from the bottom margin to the baseline of the footer
	headsep=0.5cm, % Space from the top margin to the baseline of the header
	%showframe, % Uncomment to show how the type block is set on the page
}

%----------------------------------------------------------------------------------------
%	FONTS
%----------------------------------------------------------------------------------------

\usepackage[utf8]{inputenc} % Required for inputting international characters
\usepackage[T1]{fontenc} % Output font encoding for international characters

\usepackage[default]{raleway}
%\usepackage[defaultsans]{droidsans}
%\usepackage{cmbright}

\renewcommand*\familydefault{\sfdefault} % Force the sans-serif version of any font used

%------------------------------------------------

\usepackage{fontawesome} % Required for FontAwesome icons

% Command to output an icon in a black square box
% with text to the right of the icon
% Arguments:
% 1 the FontAwesome icon name
% 2 the box size
% 3 the text
\newcommand{\iconleft}[3]{
	\vcenteredhbox{\pgfsetfillopacity{0}\colorbox{white}{\pgfsetfillopacity{1}\makebox(#2, #2){\Large\csname fa#1\endcsname}}\pgfsetfillopacity{1}}% Icon and box
	\hspace{0.1cm}% Whitespace
	\vcenteredhbox{#3}% Text
}

% Command to output an icon in a black square box
% with text to the left of the icon
% Arguments:
% 1 the FontAwesome icon name
% 2 the box size
% 3 the text
\newcommand{\iconright}[3]{
	\vcenteredhbox{#3}% Text
	\hspace{0.1cm}% Whitespace
	\vcenteredhbox{\pgfsetfillopacity{0}\colorbox{white}{\pgfsetfillopacity{1}\makebox(#2, #2){\Large\csname fa#1\endcsname}}\pgfsetfillopacity{1}}% Icon and box
}

% Command to output an icon in a black square box
% with text to the right of the icon,
% but only if the text is non empty
% Arguments:
% 1 the FontAwesome icon name
% 2 the box size
% 3 the text
\newcommand{\maybeiconleft}[3]{
	\ifx #3\empty \else
		\iconleft{#1}{#2}{#3}\\
	\fi
}

% Command to output an icon in a black square box
% with text to the left of the icon,
% but only if the text is non empty
% Arguments:
% 1 the FontAwesome icon name
% 2 the box size
% 3 the text
\newcommand{\maybeiconright}[3]{
	\ifx #3\empty \else
		\iconright{#1}{#2}{#3}\\
	\fi
}

%----------------------------------------------------------------------------------------
%	GRAPHICS DEFINITIONS
%----------------------------------------------------------------------------------------

\usepackage{tikz} % Required for creating the plots
\usetikzlibrary{shapes, backgrounds}
\tikzset{x=1cm, y=1cm} % Default tikz units

% Command to vertically centre adjacent content
\newcommand{\vcenteredhbox}[1]{% The only parameter is for the content to centre
	\begingroup%
		\setbox0=\hbox{#1}\parbox{\wd0}{\box0}%
	\endgroup%
}

\newcommand{\cvhref}[2]{%
	\href{#1}{\smash{\underline{\smash{#2}\vphantom{T}}}\vphantom{#2}}%
}



% Arguments:
% 1 name
% 2 career
% 3 image
% 4 email (optional)
% 5 phone (optional)
% 6 location (optional)
% 7 website (optional)
% 8 github (optional)
% 9 linkedin (optional)
% 10 about me section
% 11 skills section
% 12 sections
\newcommand{\cv}[9]{
	\edef\argname{\unexpanded{#1}}%
	\edef\argmotto{\unexpanded{#2}}%
	\edef\argimage{\unexpanded{#3}}%
	\edef\argemail{\unexpanded{#4}}%
	\edef\argphone{\unexpanded{#5}}%
	\edef\arglocation{\unexpanded{#6}}%
	\edef\argwebsite{\unexpanded{#7}}%
	\edef\arggithub{\unexpanded{#8}}%
	\edef\arglinkedin{\unexpanded{#9}}%
	\cv@continued
}
\newcommand{\cv@continued}[3]{
	\edef\argsummary{\unexpanded{#1}}%
	\edef\argskills{\unexpanded{#2}}%
	\edef\argsections{\unexpanded{#3}}%

	\ifx \argimage\empty
		\begin{minipage}[t]{0.35\textwidth}
			\vspace{-\baselineskip}
			{\fontsize{16}{20}\textbf{\MakeUppercase{\argname}}}
			\vspace{6pt}\\
			{\Large \argmotto}
		\end{minipage}
		\hfill
		\begin{minipage}[t]{0.32\textwidth}
			\vspace{-\baselineskip}
			\maybeiconleft{Envelope}{11}{\argemail}
			\maybeiconleft{Phone}{11}{\argphone}
			\maybeiconleft{MapMarker}{11}{\arglocation}
		\end{minipage}
		\begin{minipage}[t]{0.23\textwidth}
			\vspace{-\baselineskip}
			\maybeiconleft{Globe}{11}{\argwebsite}
			\maybeiconleft{Github}{11}{\arggithub}
			\maybeiconleft{LinkedinSquare}{11}{\arglinkedin}
		\end{minipage}
	\else
		\begin{minipage}[t][][b]{0.18\textwidth}
			\vspace{-\baselineskip}
			\includegraphics[width=\textwidth]{\argimage}
		\end{minipage}
		\hspace{0.03\textwidth}
		\hfill
		\begin{minipage}[t]{0.35\textwidth}
			\vspace{0.01\textwidth}
			\centering
			{\fontsize{16}{20}\textbf{\MakeUppercase{\argname}}}
			\vspace{6pt}\\
			{\Large \argmotto}
		\end{minipage}
		\hfill
		\begin{minipage}[t]{0.32\textwidth}
			\vspace{-\baselineskip}
			\begin{flushright}
				\maybeiconright{Globe}{11}{\argwebsite}
				\maybeiconright{Github}{11}{\arggithub}
				\maybeiconright{Phone}{11}{\argphone}
				\maybeiconright{Envelope}{11}{\argemail}
				\maybeiconright{LinkedinSquare}{11}{\arglinkedin}
				\maybeiconright{MapMarker}{11}{\arglocation}
			\end{flushright}
		\end{minipage}
	\fi

	\ifx \argsummary\empty
		% show just the skills full width, if there are any
		\argskills
	\else
		\ifx \argskills\empty
			% show just the summary full width
			\argsummary
		\else
			% show summary and skills side by side
			\begin{minipage}[t]{0.4\textwidth}
				\argsummary
			\end{minipage}
			\hfill
			\begin{minipage}[t]{0.55\textwidth}
				\argskills
			\end{minipage}
		\fi
	\fi

	\argsections
}


\usepackage{longtable} % Required for tables that span multiple pages
\setlength{\LTpre}{0pt} % Remove default whitespace before longtable
\setlength{\LTpost}{0pt} % Remove default whitespace after longtable

\setlength{\tabcolsep}{0pt} % No spacing between table columns

% Environment to hold a new list of entries
\newenvironment{@entrylist}{
	\begin{longtable}[H]{l l}
}{
	\end{longtable}
}

\def\Vhrulefill{\leavevmode\leaders\hrule height 0.7ex depth \dimexpr0.4pt-0.7ex\hfill\kern0pt}

% Arguments:
% 1 any non-empty string makes the title be rendered as in a subsection
% 2 the actual text to render as title
\newcommand{\@titlesectionorsubsection}[2]{
	\def\temp{#1}\ifx\temp\empty
		\MakeUppercase{\textbf{#2}} \hspace{4pt} \Vhrulefill \\%
	\else
		\textbf{#2} \\%
	\fi
}

% Arguments:
% 1 any non-empty string makes the title be rendered as in a subsection
% 2 title
% 3 description (optional)
% 4 entries
% 5 subsections
\newcommand{\@section}[5]{
	\begin{minipage}{\textwidth}
		\@titlesectionorsubsection{#1}{#2}
		\if #3\empty \else
			#3~\\
		\fi
		\ifx #4\empty \else
			\begin{@entrylist}
				#4
			\end{@entrylist}
		\fi
		#5
	\end{minipage}
}
\newcommand{\sectioneducation}{\@section}
\newcommand{\sectionjob}{\@section}
\newcommand{\sectionproject}{\@section}

% Arguments:
% 1 any non-empty string makes the title be rendered as in a subsection
% 2 title
% 3 description (optional)
% 4 entries
% 5 subsections
\newcommand{\@sectionlist}[5]{
	\begin{minipage}{\textwidth}
		\vspace{\baselineskip}
		\@titlesectionorsubsection{#1}{#2}
		\ifx #3\empty \else
			\\[-8pt]
			#3
			\\[-4pt]
		\fi
		\ifx #4\empty \else
			\begin{itemize}[noitemsep,topsep=0pt,parsep=0pt,partopsep=0pt,leftmargin=9pt]
				#4
			\end{itemize}
		\fi
		#5
		\vspace{12pt}
	\end{minipage}
}
\newcommand{\sectionaward}{\@sectionlist}
\newcommand{\sectiontalk}{\@sectionlist}

% Arguments:
% 1 title
% 2 description
\newcommand{\@sectionelement}[2]{
	\vspace{\baselineskip}
	\vspace{4pt}
	\@titlesectionorsubsection{}{#1}
	#2 \hfill \\
	\vspace{4pt}
}
\newcommand{\sectionsummary}{\@sectionelement}
\newcommand{\sectionskills}{\@sectionelement}

% Arguments:
% 1 leftmost "when" text (optional)
% 2 leftmost "tags" text (optional)
% 3 bold entry "heading"
% 4 semi-bold italic right-aligned entry qualifier (optional)
% 5 entry "description" text
\newcommand{\@entry}[5]{
	\parbox[t]{0.16\textwidth}{% 16% of the text width of the page
		\raggedright{
			\small
			\def\temp{#1}\ifx\temp\empty
				\footnotesize{#2}
			\else
				#1
				\def\temp{#2}\ifx\temp\empty\else
					\vspace{3pt}
					\linebreak
					\footnotesize{#2}
				\fi\\
			\fi
		}
	}%
	\parbox[t]{0.03\textwidth}{% 3% of space
		\hfill
	}
	&\parbox[t]{0.81\textwidth}{% 81% of the text width of the page
		\raggedright{
			\textbf{#3}% Entry heading text
			\hspace{2pt}
			\hfill% Horizontal whitespace
			\mbox{}~\hfill{}
			{\footnotesize \emph{\fontseries{sb}\selectfont \mbox{#4}}}\\% Right-aligned entry qualifier text
			#5 % Entry description text
		}
	}\\\\
}

% Arguments:
% 1 degree
% 2 institution
% 3 when
% 4 grade (optional)
% 5 details (optional)
\newcommand{\itemeducation}[5]{
	\@entry{#3}{#4}{#1}{#2}{#5}
}

% Arguments:
% 1 role
% 2 company
% 3 where (optional)
% 4 when
% 5 topics (optional)
% 6 details (optional)
\newcommand{\itemjob}[6]{
	\@entry{#4}{#5}{#1}{
		#2%
		\if #2\empty \else
			\if #3\empty \else
				,
			\fi
		\fi
		#3
	}{#6}
}

% Arguments:
% 1 name
% 2 technologies
% 3 links (optional)
% 4 when (optional)
% 5 details (optional)
\newcommand{\itemproject}[5]{
	\@entry{#4}{#2}{#1}{#3}{#5}
}

% Arguments:
% 1 name
% 2 when
% 3 grade (optional)
\newcommand{\itemaward}[3]{
	\item
	\ifx #3\empty \else
		\textbf{#3},
	\fi
	\emph{#1},
	#2
}

% Arguments:
% 1 name
% 2 event
% 3 when
\newcommand{\itemtalk}[3]{
	\item
	\textbf{#1},
	\emph{#2},
	#3
}
