# Pluma API
The goal of PAPI is to be an HTTP api for posting, pulling and editing BLOG data. Ultimately this will be used in combination with my website.

## Technologies
Hosting: Railway
Web Server: Rust Axum
ORM: Diesel-rs
Database: Postgresql

## Goals and Expectations
- [X] CRUD Operations for the actual blog bit of things
  - [X] POST
  - [X] GET
  - [X] PUT/PATCH
  - [X] DELETE
- [ ] Pagination / Filtering
- [ ] Authentication / Authorization (JWT)
- [ ] Rate Limiting and Abuse Prevention
- [ ] Search and Advanced Filtering
- [ ] Media Uploads
- [ ] Versioning to manage uploads
- [ ] Webhooks and event driven features(Notify end users when something is added to the feed?)
- [ ] Caching of posts that have been asked for recently
- [ ] User Interaction(Add features like comments/likes/shares
- [ ] Localization
- [ ] Analytics
