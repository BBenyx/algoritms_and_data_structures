library(rmarkdown)
library(tools)

args <- commandArgs(trailingOnly = TRUE)

if (length(args) < 1) {
    stop("Analysable file name parameter missing!")
}

# Creating csv absolute path to work with the bash script
csv_file <- normalizePath(
  commandArgs(trailingOnly = TRUE)[1],
  mustWork = TRUE
)

file_name <- file_path_sans_ext(basename(csv_file))

rmarkdown::render(
  "analysis/report.rmd",
  output_file = paste0(file_name, "_report.pdf"),
  output_dir = "reports",
  params = list(
    csv_file = csv_file,
    report_title = paste0(file_name, "_report")
  ),
  envir = new.env()
)