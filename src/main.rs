// Structure to represent a single verse
#[derive(Debug, Clone)]
struct Verse {
    book: String,
    chapter: u32,
    verse_number: u32,
    text: String,
}

// Structure to represent a chapter
#[derive(Debug, Clone)]
struct Chapter {
    number: u32,
    verses: Vec<Verse>,
}

// Structure to represent a book
#[derive(Debug, Clone)]
struct Book {
    name: String,
    #[allow(dead_code)]
    testament: Testament,
    chapters: Vec<Chapter>,
}

// Enum to represent testament type
#[derive(Debug, Clone, PartialEq)]
enum Testament {
    Old,
    New,
}

// Structure to represent the entire Bible
#[derive(Debug, Clone)]
struct Bible {
    books: Vec<Book>,
}

impl Bible {
    // Method to get a specific verse
    #[allow(dead_code)]
    fn get_verse(&self, book_name: &str, chapter: u32, verse: u32) -> Option<&Verse> {
        for book in &self.books {
            if book.name == book_name {
                if let Some(chapter) = book.chapters.iter().find(|c| c.number == chapter) {
                    return chapter.verses.iter().find(|v| v.verse_number == verse);
                }
            }
        }
        None
    }
    
    // Method to get an entire chapter
    fn get_chapter(&self, book_name: &str, chapter: u32) -> Option<&Chapter> {
        for book in &self.books {
            if book.name == book_name {
                return book.chapters.iter().find(|c| c.number == chapter);
            }
        }
        None
    }
    
    // Method to search for text in all verses
    fn search(&self, query: &str) -> Vec<&Verse> {
        let query = query.to_lowercase();
        let mut results = Vec::new();
        
        for book in &self.books {
            for chapter in &book.chapters {
                for verse in &chapter.verses {
                    if verse.text.to_lowercase().contains(&query) {
                        results.push(verse);
                    }
                }
            }
        }
        
        results
    }
}

use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

impl Bible {
    fn from_directories(old_testament_path: &Path, new_testament_path: &Path) -> io::Result<Self> {
        let mut bible = Bible { books: Vec::new() };
        
        // Get the standard book order
        let book_order = get_standard_book_order();
        
        // Read Old Testament books
        Self::read_testament_books(&mut bible, old_testament_path, Testament::Old)?;
        
        // Read New Testament books
        Self::read_testament_books(&mut bible, new_testament_path, Testament::New)?;
        
        // Sort books according to the standard biblical order
        bible.books.sort_by(|a, b| {
            let a_order = book_order.get(&a.name).unwrap_or(&999);
            let b_order = book_order.get(&b.name).unwrap_or(&999);
            a_order.cmp(b_order)
        });
        
        Ok(bible)
    }
    
    fn read_testament_books(bible: &mut Bible, testament_path: &Path, testament: Testament) -> io::Result<()> {
        for entry in fs::read_dir(testament_path)? {
            let entry = entry?;
            let file_path = entry.path();
            
            // Skip .DS_Store files
            if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
                if file_name == ".DS_Store" {
                    continue;
                }
            }
            
            if file_path.is_file() {
                if let Some(book_name) = file_path.file_stem().and_then(|s| s.to_str()) {
                    let book = Self::parse_book_file(&file_path, book_name.to_string(), testament.clone())?;
                    bible.books.push(book);
                }
            }
        }
        
        Ok(())
    }
    
    fn parse_book_file(file_path: &Path, book_name: String, testament: Testament) -> io::Result<Book> {
        // Try to open the file with more lenient encoding handling
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file {:?}: {}", file_path, e);
                return Err(e);
            }
        };
        
        let mut book = Book {
            name: book_name,
            testament,
            chapters: Vec::new(),
        };
        
        // Read the file line by line, handling potential encoding issues
        let reader = io::BufReader::new(file);
        for (line_idx, line_result) in reader.lines().enumerate() {
            let line = match line_result {
                Ok(line) => line,
                Err(e) => {
                    eprintln!("Error reading line {} in {:?}: {}", line_idx + 1, file_path, e);
                    continue;  // Skip this line and try to continue with the next
                }
            };
            
            // Assume the format is "chapter:verse_number verse_text"
            // You might need to adjust this based on the actual format of your files
            if let Some((reference, text)) = line.split_once(' ') {
                if let Some((chapter_str, verse_str)) = reference.split_once(':') {
                    let chapter_num = chapter_str.parse::<u32>().unwrap_or(0);
                    let verse_num = verse_str.parse::<u32>().unwrap_or(0);
                    
                    // Ensure we have enough chapters
                    while book.chapters.len() < chapter_num as usize {
                        book.chapters.push(Chapter {
                            number: book.chapters.len() as u32 + 1,
                            verses: Vec::new(),
                        });
                    }
                    
                    // Add verse to the chapter
                    let chapter_idx = chapter_num as usize - 1;
                    if chapter_idx < book.chapters.len() {
                        book.chapters[chapter_idx].verses.push(Verse {
                            book: book.name.clone(),
                            chapter: chapter_num,
                            verse_number: verse_num,
                            text: text.to_string(),
                        });
                    }
                } else {
                    // If there's no chapter:verse format, log an error or handle accordingly
                    eprintln!("Warning: Malformed verse at line {}: {}", line_idx + 1, line);
                }
            }
        }
        
        Ok(book)
    }
}

// Function that provides the standard biblical order
fn get_standard_book_order() -> HashMap<String, usize> {
    let books = vec![
        // Old Testament
        "Genesis", "Exodus", "Leviticus", "Numbers", "Deuteronomy",
        "Joshua", "Judges", "Ruth", "1Samuel", "2Samuel",
        "1Kings", "2Kings", "1Chronicles", "2Chronicles",
        "Ezra", "Nehemiah", "Esther", "Job", "Psalms", "Proverbs",
        "Ecclesiastes", "SongofSolomon", "Isaiah", "Jeremiah",
        "Lamentations", "Ezekiel", "Daniel", "Hosea", "Joel", "Amos",
        "Obadiah", "Jonah", "Micah", "Nahum", "Habakkuk", "Zephaniah",
        "Haggai", "Zechariah", "Malachi",
        
        // New Testament
        "Matthew", "Mark", "Luke", "John", "Acts",
        "Romans", "1Corinthians", "2Corinthians", "Galatians", "Ephesians",
        "Philippians", "Colossians", "1Thessalonians", "2Thessalonians",
        "1Timothy", "2Timothy", "Titus", "Philemon", "Hebrews",
        "James", "1Peter", "2Peter", "1John", "2John", "3John",
        "Jude", "Revelation"
    ];
    
    let mut order_map = HashMap::new();
    for (index, book) in books.iter().enumerate() {
        order_map.insert(book.to_string(), index);
    }
    
    order_map
}

use egui::{ComboBox, ScrollArea, TextEdit};

struct BibleApp {
    bible: Bible,
    selected_book: String,
    selected_chapter: u32,
    chapter_text: String,
    current_chapter_verses: Vec<Verse>,
    search_query: String,
    search_results: Vec<Verse>,
    navigate_to: Option<(String, u32)>,
}

impl BibleApp {
    fn new(bible: Bible) -> Self {
        let default_book = bible.books.first().map_or("Genesis".to_string(), |b| b.name.clone());
        let default_chapter = 1;
        
        // Get all verses for the default chapter
        let mut current_chapter_verses = Vec::new();
        let mut chapter_text = String::new();
        
        if let Some(chapter) = bible.get_chapter(&default_book, default_chapter) {
            current_chapter_verses = chapter.verses.clone();
            
            // Format the chapter text
            for verse in &chapter.verses {
                chapter_text.push_str(&format!("{} {}\n\n", verse.verse_number, verse.text));
            }
        }
        
        Self {
            bible,
            selected_book: default_book,
            selected_chapter: default_chapter,
            chapter_text,
            current_chapter_verses,
            search_query: "".to_string(),
            search_results: Vec::new(),
            navigate_to: None,
        }
    }
    
    fn update_chapter_display(&mut self) {
        // Clear previous content
        self.chapter_text.clear();
        self.current_chapter_verses.clear();
        
        // Get the selected chapter
        if let Some(chapter) = self.bible.get_chapter(&self.selected_book, self.selected_chapter) {
            self.current_chapter_verses = chapter.verses.clone();
            
            // Format the chapter text
            for verse in &chapter.verses {
                self.chapter_text.push_str(&format!("{} {}\n\n", verse.verse_number, verse.text));
            }
        } else {
            self.chapter_text = "Chapter not found".to_string();
        }
    }
}

impl eframe::App for BibleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bible Reader");
            
            ui.horizontal(|ui| {
                // Book selection dropdown
                ComboBox::from_label("Book")
                    .selected_text(&self.selected_book)
                    .show_ui(ui, |ui| {
                        for book in &self.bible.books {
                            ui.selectable_value(&mut self.selected_book, book.name.clone(), &book.name);
                        }
                    });
                
                // Find the selected book to get chapter count
                if let Some(book) = self.bible.books.iter().find(|b| b.name == self.selected_book) {
                    let chapter_count = book.chapters.len() as u32;
                    
                    // Chapter selection dropdown
                    ComboBox::from_label("Chapter")
                        .selected_text(self.selected_chapter.to_string())
                        .show_ui(ui, |ui| {
                            for chapter_num in 1..=chapter_count {
                                ui.selectable_value(&mut self.selected_chapter, chapter_num, chapter_num.to_string());
                            }
                        });
                }
                
                // Update chapter text when selection changes
                if ui.button("Go").clicked() {
                    self.update_chapter_display();
                }
            });
            
            // Display the selected chapter
            ui.add_space(10.0);
            ui.heading(format!("{} Chapter {}", self.selected_book, self.selected_chapter));
            ui.separator();
            
            // Use ScrollArea for displaying the chapter text
            ScrollArea::vertical()
                .max_height(300.0)
                .id_salt("chapter_scroll")
                .show(ui, |ui| {
                    ui.add(TextEdit::multiline(&mut self.chapter_text)
                         .desired_width(f32::INFINITY)
                         .desired_rows(10)
                         .interactive(false)
                         .margin(egui::vec2(8.0, 8.0)));
                });
                
            ui.separator();
            
            // Search functionality
            ui.add_space(20.0);
            ui.heading("Search");
            
            ui.horizontal(|ui| {
                let text_edit = TextEdit::singleline(&mut self.search_query)
                    .hint_text("Search for text...")
                    .desired_width(300.0);
                
                ui.add(text_edit);
                
                if ui.button("Search").clicked() {
                    // Perform search and convert results
                    let results = self.bible.search(&self.search_query);
                    self.search_results = results.iter().map(|v| (*v).clone()).collect();
                }
            });
            
            // Display search results
            if !self.search_results.is_empty() {
                ui.add_space(10.0);
                ui.label(format!("Found {} results:", self.search_results.len()));
                
                ScrollArea::vertical()
                    .max_height(200.0)
                    .id_salt("search_results_scroll")  // Use id_salt instead of id_source
                    .show(ui, |ui| {
                        for result in &self.search_results {
                            let book = result.book.clone();
                            let chapter = result.chapter;
                            
                            if ui.selectable_label(false, format!("{} {}:{} - {}", 
                                                            result.book, 
                                                            result.chapter, 
                                                            result.verse_number, 
                                                            result.text)).clicked() {
                                // Instead of updating immediately, store the navigation target
                                self.navigate_to = Some((book, chapter));
                            }
                        }
                    });
            }
            
            // After all UI elements are drawn, check if we need to navigate
            if let Some((book, chapter)) = self.navigate_to.take() {
                self.selected_book = book;
                self.selected_chapter = chapter;
                self.update_chapter_display();
            }
        });
    }
}

use eframe::{egui, NativeOptions};

fn main() -> eframe::Result<()> {
    // Define paths to testament directories
    let old_testament_path = Path::new("old_testament");
    let new_testament_path = Path::new("new_testament");

    
    // Load the Bible data
    let bible = match Bible::from_directories(old_testament_path, new_testament_path) {
        Ok(bible) => bible,
        Err(e) => {
            eprintln!("Error loading Bible: {}", e);
            return Ok(());
        }
    };
    
    // Debug: Print books in the order they'll appear in the app
    println!("Books in biblical order:");
    for (i, book) in bible.books.iter().enumerate() {
        println!("{:2}. {}", i + 1, book.name);
    }
    
    // Set up window options
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    // Create the Bible App with initial chapter display
    let mut app = BibleApp::new(bible);
    app.update_chapter_display(); // Initialize with first chapter content
    
    // Run the app
    eframe::run_native(
        "Bible App",
        options,
        Box::new(|_cc| Ok(Box::new(app)))
    )?;
    Ok(())
}
