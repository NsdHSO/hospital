# Hospital Emergency Management System

## 1. Project Overview

The Hospital Emergency Management System is a comprehensive solution for coordinating emergency medical services. Built with Rust, this system provides robust tools for managing hospitals, ambulances, emergencies, triages, and users to facilitate efficient emergency response coordination.

### Key Features

- **Emergency Case Management**: Track and manage emergency cases with detailed information including location (using geohashing), status, and severity level.
- **Ambulance Dispatch**: Assign ambulances to emergencies and monitor their status in real-time.
- **Hospital Resource Management**: Track hospital capacity, resources, and ambulance fleets.
- **Triage System**: Prioritize emergencies based on severity levels using a standardized triage system.
- **User Authentication**: Secure access control for different user roles in the system.
- **REST API**: Comprehensive API for integrating with other healthcare systems.
- **Dashboard**: Real-time visualization of emergency situations and resource allocation.

### Technology Stack

- **Backend**: Rust with Actix-web framework
- **Database**: PostgreSQL with Sea-ORM
- **Authentication**: JWT-based authentication
- **Documentation**: OpenAPI/Swagger for API documentation
- **Containerization**: Docker support
- **Deployment**: Fly.io configuration included

## 2. System Architecture

The system follows a modular architecture with clear separation of concerns:

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  HTTP API   │────▶│  Services   │────▶│ Data Access │
│  (Actix-web)│     │  (Business  │     │  (Sea-ORM)  │
└─────────────┘     │   Logic)    │     └─────────────┘
                    └─────────────┘            │
                           ▲                   ▼
                           │            ┌─────────────┐
                           └────────────│  Database   │
                                        │ (PostgreSQL)│
                                        └─────────────┘
```

### Component Relationships

```
┌─────────────┐     owns     ┌─────────────┐
│   Hospital  │◄────────────▶│  Ambulance  │
└─────────────┘              └─────────────┘
       ▲                            ▲
       │                            │
       │ assigned to                │ assigned to
       │                            │
       ▼                            ▼
┌─────────────┐     linked to ┌─────────────┐
│  Emergency  │◄────────────▶│   Triage    │
└─────────────┘              └─────────────┘
```

### Core Components

1. **Routes Layer**: HTTP endpoints implemented with Actix-web
2. **Services Layer**: Business logic that processes requests and manages application state
3. **Entity Layer**: Data models representing database tables
4. **Repository Layer**: Database interactions using Sea-ORM

## 3. Database Schema

The system uses a relational database with the following key entities and relationships:

### Core Entities

- **Hospital**: Information about healthcare facilities
- **Ambulance**: Emergency vehicles linked to specific hospitals
- **Emergency**: Emergency cases with location, description, and status
- **Triage**: Classification system for prioritizing emergency cases
- **Person**: Comprehensive information about individuals (patients, medical staff, etc.)
- **User**: System users with authentication details

### Key Relationships

- **Hospital to Ambulance**: One-to-many (a hospital owns multiple ambulances)
- **Hospital to Emergency**: One-to-many (emergencies can be assigned to hospitals)
- **Ambulance to Emergency**: One-to-many (an ambulance can handle multiple emergencies over time)
- **Emergency to Triage**: Many-to-one (emergencies are classified using triage levels)
- **Person to Emergency**: One-to-many (a person can be involved in multiple emergencies)
- **Person to Hospital**: Many-to-one (a person can be associated with a hospital, e.g., as staff or patient)

### Entity Relationship Diagram (ERD)

```
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│   Hospital    │      │   Ambulance   │      │   Emergency   │
├───────────────┤      ├───────────────┤      ├───────────────┤
│ id (PK)       │──┐   │ id (PK)       │──┐   │ id (PK)       │
│ name          │  │   │ name          │  │   │ emergency_date│
│ location      │  │   │ plate_number  │  │   │ description   │
│ address       │  │   │ status        │  │   │ latitude      │
│ created_at    │  │   │ id_hospital(FK)│◄─┘   │ longitude     │
│ updated_at    │  │   │ created_at    │      │ geohash       │
└───────────────┘  │   │ updated_at    │      │ status        │
         ▲         │   └───────────────┘      │ id_ambulance(FK)│
         │         │                          │ id_triage(FK) │
         │         │                          │ id_hospital(FK)│◄─┘
         │         │                          │ id_person(FK) │
         │         │                          │ created_at    │
         │         │                          │ updated_at    │
┌───────────────┐  │                          └───────────────┘
│    Triage     │  │                                  ▲
├───────────────┤  │                                  │
│ id (PK)       │  │                                  │
│ name          │  │                                  │
│ description   │  │                                  │
│ level         │──┘                                  │
│ created_at    │                                     │
│ updated_at    │─────────────────────────────────────┘
└───────────────┘                                      
                                                       
┌───────────────┐                                      
│    Person     │                                      
├───────────────┤                                      
│ id (PK)       │──────────────────────────────────────┘
│ first_name    │                                       
│ last_name     │                                       
│ date_of_birth │                                       
│ gender        │                                       
│ address       │                                       
│ phone         │                                       
│ email         │                                       
│ blood_type    │                                       
│ medical_history│                                      
│ id_hospital(FK)│─────────────────────────────────────┘
│ created_at    │                                       
│ updated_at    │                                       
└───────────────┘                                       
```

## 4. Setup and Installation

### Prerequisites

- Rust (1.70.0 or later)
- PostgreSQL (14.0 or later)
- Docker (optional, for containerized deployment)

### Local Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/hospital-emergency-system.git
   cd hospital-emergency-system
   ```

2. **Set up environment variables**
   Create a `.env` file in the project root with the following variables:
   ```
   DATABASE_URL=postgres://username:password@localhost:5432/hospital_db
   HOST=127.0.0.1
   PORT=8080
   JWT_SECRET=your_jwt_secret_key
   ```

3. **Set up the database**
   ```bash
   # Create the database
   createdb hospital_db
   
   # Run migrations
   cargo run --bin migration
   ```

4. **Run the application**
   ```bash
   cargo run
   ```

5. **Access the application**
   - API: http://localhost:8080
   - Swagger UI: http://localhost:8080/swagger-ui/

### Docker Deployment

1. **Build the Docker image**
   ```bash
   docker build -t hospital-emergency-system .
   ```

2. **Run the container**
   ```bash
   docker run -p 8080:8080 --env-file .env hospital-emergency-system
   ```

### Deployment with Fly.io

1. **Install Fly CLI**
   ```bash
   curl -L https://fly.io/install.sh | sh
   ```

2. **Deploy the application**
   ```bash
   fly deploy
   ```

## 5. API Endpoints

The system provides RESTful API endpoints for all core entities. Here's a summary of the main endpoints:

### Hospital Endpoints

- `GET /v1/hospital` - List all hospitals
- `GET /v1/hospital/{id}` - Get hospital details
- `POST /v1/hospital` - Create a new hospital
- `PUT /v1/hospital/{id}` - Update hospital information
- `DELETE /v1/hospital/{id}` - Delete a hospital

### Ambulance Endpoints

- `GET /v1/ambulance` - List all ambulances
- `GET /v1/ambulance/{id}` - Get ambulance details
- `POST /v1/ambulance` - Create a new ambulance
- `PUT /v1/ambulance/{id}` - Update ambulance information
- `DELETE /v1/ambulance/{id}` - Delete an ambulance

### Emergency Endpoints

- `GET /v1/emergency` - List all emergencies
- `GET /v1/emergency/{id}` - Get emergency details
- `POST /v1/emergency` - Create a new emergency
- `PUT /v1/emergency/{id}` - Update emergency information
- `DELETE /v1/emergency/{id}` - Delete an emergency

### Triage Endpoints

- `GET /v1/triage` - List all triage categories
- `GET /v1/triage/{id}` - Get triage details
- `POST /v1/triage` - Create a new triage category
- `PUT /v1/triage/{id}` - Update triage information
- `DELETE /v1/triage/{id}` - Delete a triage category

### User Endpoints

- `POST /v1/user/register` - Register a new user
- `POST /v1/user/login` - Authenticate user and get token
- `GET /v1/user/profile` - Get user profile information
- `PUT /v1/user/profile` - Update user profile

### Emergency Allocation Endpoints

- `POST /v1/emergency/allocate` - Allocate an ambulance to an emergency
- `GET /v1/emergency/allocation/{id}` - Get allocation details

For detailed API documentation, access the Swagger UI at `/swagger-ui/` when the application is running.

## 6. Development Guidelines

### Project Structure

```
hospital/
├── migration/            # Database migrations
├── src/
│   ├── auth/             # Authentication logic
│   ├── components/       # Business components
│   │   ├── ambulance/    # Ambulance-related functionality
│   │   ├── card/         # Card-related functionality
│   │   ├── dashboard/    # Dashboard-related functionality
│   │   ├── emergency/    # Emergency-related functionality
│   │   ├── hospital/     # Hospital-related functionality
│   │   ├── resource/     # Resource-related functionality
│   │   ├── triage/       # Triage-related functionality
│   │   └── user/         # User-related functionality
│   ├── db/               # Database configuration
│   ├── entity/           # Entity definitions (Sea-ORM models)
│   ├── error_handler/    # Error handling
│   ├── http_response/    # HTTP response utilities
│   ├── open_api/         # OpenAPI/Swagger configuration
│   ├── shared/           # Shared utilities
│   ├── tests/            # Unit and integration tests
│   └── utils/            # Utility functions
├── .env                  # Environment variables
├── Cargo.toml            # Project dependencies
├── Dockerfile            # Docker configuration
├── fly.toml              # Fly.io configuration
└── README.md             # Project documentation
```

### Component Structure

Each component typically follows this structure:
- `mod.rs` - Module exports
- `routes.rs` - HTTP endpoints
- `services.rs` - Business logic
- `model.rs` - Database models (if needed)

### Coding Standards

1. **Error Handling**: Use the custom error handler for consistent error responses
2. **Validation**: Validate input data before processing
3. **Documentation**: Document all public functions and modules
4. **Testing**: Write unit tests for business logic and integration tests for API endpoints

### Contribution Guidelines

1. **Fork the repository** and create a feature branch
2. **Follow the code style** of the project
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Submit a pull request** with a clear description of the changes

### Common Development Tasks

- **Run tests**: `cargo test`
- **Run linter**: `cargo clippy`
- **Format code**: `cargo fmt`
- **Generate documentation**: `cargo doc --open`

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- The Rust community for their excellent libraries and tools
- Contributors to this project

For additional support, please open an issue on the project repository.

---

## 7. Component Relationships Diagram

This UML diagram represents the relationships between the different components of the Hospital Emergency Management System:

```plantuml
@startuml

package "Hospital Management System" {
  [Hospital Component] as Hospital
  [Ambulance Component] as Ambulance
  [Emergency Component] as Emergency
  [Patient Component] as Patient
  [Dashboard Component] as Dashboard
  [Card Component] as Card
}

' Component relationships
Hospital "1" *-- "many" Ambulance : owns >
Hospital "1" *-- "many" Patient : manages >
Ambulance "1" --> "many" Emergency : dispatched to >
Emergency "1" o-- "many" Patient : involves >
Patient "many" -- "many" Emergency : associated with >
Dashboard "1" *-- "many" Card : displays >

@enduml
```
![img.png](img.png)

In this UML diagram:
* `*--` represents composition/ownership relationship
* `o--` represents aggregation relationship
* `-->` represents directed association
* `--` represents bidirectional association
* Numbers indicate cardinality (one-to-many, many-to-many, etc.)

### Component Details

1. **Hospital Component**
   - Manages hospital information and resources
   - Owns and manages ambulances
   - Services: Hospital management, resource tracking

2. **Ambulance Component**
   - Manages ambulance fleet and dispatch
   - Belongs to a specific hospital
   - Services: Ambulance status tracking, dispatch management

3. **Emergency Component**
   - Handles emergency case information
   - Connects patients with ambulances and hospitals
   - Services: Emergency creation, status updates, ambulance assignment

4. **Patient Component**
   - Manages patient information
   - Connected to emergency cases
   - Services: Patient registration, medical history

5. **Dashboard Component**
   - Provides system visualization
   - Services: Dashboard configuration and management

6. **Card Component**
   - Visual elements for dashboards
   - Services: Card creation and configuration

Each component follows a modular structure with routes, services, and data models, promoting separation of concerns and maintainability.

## 8. Target Architecture: Comprehensive Hospital Management System

This section outlines the target architecture for a comprehensive Hospital Management System (HMS) including billing, patient management, pharmacy, laboratory, and other essential hospital operations.

### System Components Overview

```
┌──────────────────────────────────────────────────────────────────────────┐
│                  COMPREHENSIVE HOSPITAL MANAGEMENT SYSTEM                  │
└──────────────────────────────────────────────────────────────────────────┘
                                     │
     ┌────────────┬─────────────┬────┴─────┬─────────────┬─────────────┐
     ▼            ▼             ▼          ▼             ▼             ▼
┌─────────┐ ┌─────────┐  ┌──────────┐ ┌─────────┐  ┌─────────┐  ┌──────────┐
│  CORE   │ │  ADMIN  │  │ CLINICAL │ │ANALYTICS│  │SECURITY │  │   API    │
│COMPONENTS│ │COMPONENTS│  │COMPONENTS│ │   & BI  │  │   & IAM │  │ GATEWAY  │
└─────────┘ └─────────┘  └──────────┘ └─────────┘  └─────────┘  └──────────┘
     │          │             │           │            │             │
     └──────────┴─────────────┴───────────┴────────────┴─────────────┘
```

### Core Components

```
┌──────────────────────────────────────────────────────┐
│                  CORE COMPONENTS                      │
└──────────────────────────────────────────────────────┘
                        │
    ┌─────────┬─────────┼───────────┬────────────┐
    ▼         ▼         ▼           ▼            ▼
┌────────┐┌────────┐┌────────┐ ┌──────────┐ ┌──────────┐
│Hospital ││Patient ││ Staff  │ │Emergency │ │Ambulance │
│Mgmt    ││Mgmt    ││ Mgmt   │ │Mgmt      │ │Mgmt      │
└────────┘└────────┘└────────┘ └──────────┘ └──────────┘
```

### Administrative Components

```
┌──────────────────────────────────────────────────────┐
│             ADMINISTRATIVE COMPONENTS                 │
└──────────────────────────────────────────────────────┘
                        │
    ┌─────────┬─────────┼───────────┬─────────────┐
    ▼         ▼         ▼           ▼             ▼
┌────────┐┌────────┐┌──────────┐┌──────────┐ ┌──────────┐
│Billing │││Insurance││Appointment││Inventory │ │Resource  │
│System  ││Mgmt     ││System    ││Mgmt      │ │Scheduler │
└────────┘└────────┘└──────────┘└──────────┘ └──────────┘
```

### Clinical Components

```
┌──────────────────────────────────────────────────────────┐
│                  CLINICAL COMPONENTS                      │
└──────────────────────────────────────────────────────────┘
                        │
  ┌─────────┬───────────┼───────────┬────────────┬────────┐
  ▼         ▼           ▼           ▼            ▼        ▼
┌────┐ ┌────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐┌────────┐
│EMR │ │Lab Mgmt│ │Pharmacy  │ │Radiology │ │Telemedi- ││Surgery │
│    │ │        │ │Management│ │Management│ │cine      ││Mgmt    │
└────┘ └────────┘ └──────────┘ └──────────┘ └──────────┘└────────┘
```

### Analytics & Business Intelligence

```
┌──────────────────────────────────────────────────────┐
│            ANALYTICS & BUSINESS INTELLIGENCE         │
└──────────────────────────────────────────────────────┘
                        │
         ┌──────────────┼────────────────┐
         ▼              ▼                ▼
    ┌────────┐     ┌────────┐       ┌────────┐
    │Dashboard│     │Reporting│       │Analytics│
    │        │     │System  │       │Engine  │
    └────────┘     └────────┘       └────────┘
```

### Security & Identity Management

```
┌──────────────────────────────────────────────────────┐
│           SECURITY & IDENTITY MANAGEMENT             │
└──────────────────────────────────────────────────────┘
                        │
         ┌──────────────┼────────────────┐
         ▼              ▼                ▼
    ┌────────┐     ┌────────┐       ┌────────┐
    │User    │     │Audit   │       │Compli- │
    │Auth    │     │System  │       │ance    │
    └────────┘     └────────┘       └────────┘
```

### Component Details

#### Core Components
1. **Hospital Management**
   - Manages hospital information, departments, resources, and policies
   - Central hub for hospital administration

2. **Patient Management**
   - Patient registration and demographic information
   - Medical history and patient tracking
   - Patient portal for self-service

3. **Staff Management**
   - Employee records and credentials
   - Scheduling and shift management
   - Performance tracking and payroll integration

4. **Emergency Management**
   - Emergency case handling and triage
   - Resource allocation during emergencies
   - Disaster response coordination

5. **Ambulance Management**
   - Fleet management and dispatch
   - Real-time tracking and status updates
   - Maintenance scheduling

#### Administrative Components
6. **Billing System**
   - Invoice generation and payment processing
   - Insurance claim management
   - Financial reporting and revenue cycle management

7. **Insurance Management**
   - Policy verification and eligibility checks
   - Claims processing and tracking
   - Insurance provider relationship management

8. **Appointment System**
   - Scheduling and calendar management
   - Automated reminders and notifications
   - Resource allocation for appointments

9. **Inventory Management**
   - Medical supplies tracking and ordering
   - Equipment management and maintenance
   - Automated reordering and stock alerts

#### Clinical Components
10. **Electronic Medical Records (EMR)**
    - Digital patient records and charts
    - Clinical documentation and history
    - Integration with medical devices and lab systems

11. **Laboratory Management**
    - Test ordering and specimen tracking
    - Results management and reporting
    - Quality control and compliance

12. **Pharmacy Management**
    - Medication inventory and dispensing
    - Prescription management and tracking
    - Drug interaction and allergy checking

13. **Radiology Management**
    - Imaging order management
    - PACS (Picture Archiving and Communication System) integration
    - Reporting and results distribution

14. **Telemedicine**
    - Virtual consultations and appointments
    - Secure video conferencing
    - Remote patient monitoring

#### Analytics & Reporting
15. **Dashboard**
    - Real-time visualization of key metrics
    - Customizable views for different roles
    - Operational intelligence at a glance

16. **Reporting System**
    - Standard and custom report generation
    - Regulatory and compliance reporting
    - Scheduled report distribution

17. **Analytics Engine**
    - Data mining and predictive analytics
    - Population health management
    - Resource utilization optimization

#### Security & Compliance
18. **User Authentication**
    - Secure login and access control
    - Role-based permissions
    - Multi-factor authentication

19. **Audit System**
    - Activity logging and monitoring
    - Security incident tracking
    - Compliance verification

20. **Compliance Management**
    - Regulatory requirement tracking
    - Policy enforcement and documentation
    - Certification and accreditation management

This comprehensive architecture provides a roadmap for expanding the current system into a full-featured Hospital Management System that addresses all aspects of hospital operations, from clinical care to administrative functions.

# hospital
