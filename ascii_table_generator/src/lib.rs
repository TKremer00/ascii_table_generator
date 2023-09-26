use std::fmt::Display;

#[derive(Debug)]
pub struct AsciiTable {
    column_widths: Vec<usize>,
    rows: Vec<AsciiTableRow>,
}

#[derive(Debug)]
pub struct AsciiTableRow {
    height: usize,
    columns: Vec<AsciiTableColumn>,
}

#[derive(Debug)]
pub struct AsciiTableColumn {
    height: usize,
    text: String,
}

impl AsciiTable {
    pub fn new(column_widths: Vec<usize>, rows: Vec<AsciiTableRow>) -> Self {
        Self {
            column_widths,
            rows,
        }
    }

    fn make_ascii_row(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("+")?;
        for width in &self.column_widths {
            f.write_str(&"-".repeat(*width))?;
            f.write_str("+")?;
        }
        f.write_str("\n")?;

        Ok(())
    }
}

impl AsciiTableRow {
    fn columns_to_vertical_columns(&self) -> Vec<Vec<String>> {
        let mut vertical_columns: Vec<Vec<String>> = Vec::new();
        for column in &self.columns {
            let spacing = (self.height - column.height) as f32 / 2.0;
            let mut column_vec = vec!["".to_string(); spacing.floor() as usize];
            column_vec.append(
                column
                    .text
                    .split("\n")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .as_mut(),
            );
            column_vec.append(vec!["".to_string(); spacing.ceil() as usize].as_mut());
            vertical_columns.push(column_vec);
        }

        vertical_columns
    }
}

impl Display for AsciiTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.make_ascii_row(f)?;

        for row in &self.rows {
            let vertical_columns = row.columns_to_vertical_columns();

            for i in 0..row.height {
                f.write_str("|")?;
                for j in 0..row.columns.len() {
                    let column = &vertical_columns[j][i];
                    let width = self.column_widths[j];
                    let access_space: f32 = (width - column.get_length()) as f32 / 2.0;

                    f.write_str(&" ".repeat(access_space.floor() as usize))?;
                    f.write_str(column)?;
                    f.write_str(&" ".repeat(access_space.ceil() as usize))?;
                    f.write_str("|")?;
                }
                f.write_str("\n")?;
            }

            self.make_ascii_row(f)?;
        }

        Ok(())
    }

    /*
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("+")?;
        for width in &self.column_widths {
            f.write_str(&"-".repeat(*width))?;
            f.write_str("+")?;
        }
        f.write_str("\n")?;

        for row in &self.rows {
            if row.height == 1 {
                f.write_str("|")?;
                for i in 0..row.columns.len() {
                    let column = &row.columns[i].text;
                    let width = self.column_widths[i];
                    let access_space: f32 = (width - column.get_length()) as f32 / 2.0;

                    f.write_str(&" ".repeat(access_space.floor() as usize))?;
                    f.write_str(column)?;
                    f.write_str(&" ".repeat(access_space.ceil() as usize))?;
                    f.write_str("|")?;
                }
                f.write_str("\n")?;
            } else {
                let mut columns_fixed: Vec<Vec<String>> = Vec::new();

                for column in &row.columns {
                    if column.height != row.height {
                        let spacing = (row.height - column.height) as f32 / 2.0;
                        let mut column_vec = vec!["".to_string(); spacing.floor() as usize];
                        column_vec.append(
                            column
                                .text
                                .split("\n")
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .as_mut(),
                        );
                        column_vec.append(vec!["".to_string(); spacing.ceil() as usize].as_mut());
                        columns_fixed.push(column_vec);
                        continue;
                    }

                    columns_fixed.push(
                        column
                            .text
                            .split("\n")
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>(),
                    )
                }

                for i in 0..row.height {
                    f.write_str("|")?;
                    for j in 0..row.columns.len() {
                        let column = &columns_fixed[j][i];
                        // eprintln!("{:?}", column);
                        let width = self.column_widths[j];
                        let access_space: f32 = (width - column.get_length()) as f32 / 2.0;

                        f.write_str(&" ".repeat(access_space.floor() as usize))?;
                        f.write_str(column)?;
                        f.write_str(&" ".repeat(access_space.ceil() as usize))?;
                        f.write_str("|")?;
                    }
                    f.write_str("\n")?;
                }
            }

            f.write_str("+")?;
            for width in &self.column_widths {
                f.write_str(&"-".repeat(*width))?;
                f.write_str("+")?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
    */
}

pub fn make_ascii_table(table_data: Vec<Vec<String>>) -> AsciiTable {
    assert!(table_data.len() > 0);

    let mut rows = Vec::new();
    let mut column_widths = vec![0; table_data[0].len()];

    for row in &table_data {
        rows.push(convert_to_row(row.clone()));
    }

    for i in 0..table_data[0].len() {
        let columns = &table_data.iter().map(|x| x[i].clone()).collect();
        column_widths[i] = determine_column_width(&columns);
    }

    AsciiTable::new(column_widths, rows)
}

fn convert_to_row(row: Vec<String>) -> AsciiTableRow {
    let height = determine_row_height(&row);
    AsciiTableRow {
        height,
        columns: row.into_iter().map(|x| convert_to_column(x)).collect(),
    }
}

fn convert_to_column(text: String) -> AsciiTableColumn {
    let height = determine_column_height(&text);
    AsciiTableColumn { height, text }
}

fn determine_column_width(columns: &Vec<String>) -> usize {
    let mut biggest_column_width = usize::MIN;

    for ref column in columns {
        let column_width = column.get_length();
        if biggest_column_width < column_width {
            biggest_column_width = column_width;
        }
    }

    biggest_column_width + 2
}

fn determine_column_height(column: &String) -> usize {
    column.split('\n').count()
}

fn determine_row_height(columns: &Vec<String>) -> usize {
    let mut highest_column = usize::MIN;

    for column in columns {
        let column_height = determine_column_height(column);
        if highest_column < column_height {
            highest_column = column_height;
        }
    }

    highest_column
}

trait StringLength {
    fn get_length(&self) -> usize;
}

impl StringLength for String {
    fn get_length(&self) -> usize {
        if !self.contains('\n') {
            return self.chars().count();
        }

        // NOTE: could return None, but other validation should have already picked this up.
        self.split('\n')
            .map(|x| x.chars().count())
            .reduce(usize::max)
            .unwrap()
    }
}
