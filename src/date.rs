    // Set your target date (e.g., January 15, 2025, 12:00 PM UTC)
    let data = Local.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();

    // Current date and time
    let now: DateTime<Local> = Local::now();

    // Calculate the difference
    let duration = now.signed_duration_since(data);

    // Convert the duration to seconds
    let seconds_passed = duration.num_seconds();
