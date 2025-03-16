# Assessment Overview
 
Your task is to build a basic HTTP API server to store referral codes from users.
 
On the frontend, you can navigate to "./src/app/main/widgets/ReferralWidgets" and locate the function "storeReferralDataToDB". This function is responsible for calling the API to save referral codes. Additionally, check the file named "env.example" for backend API endpoint configuration—you can modify it as needed.
 
 
 
# Requirements
 

    Implement an HTTP web server that serves an API to save referral codes.
    Set up a database to store the referral codes.
    Use Docker and Docker Compose to containerize all those web servers and the databases.

 
 
# Restrictions
 

    Each user can generate a maximum of 10 referrals.
    Referral codes must be unique, and users cannot reuse previously used codes.
    You may use any programming language for the web server.
    Any database system is allowed (e.g., SQLite, MySQL, PostgreSQL, MongoDB, or Redis).
    Feel free to use prebuilt Docker images.

 
 
# Bonus Points
 
Employing microservices architecture will be considered a significant advantage.
  