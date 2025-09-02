// MongoDB initialization script for development environment

// Switch to the application database
db = db.getSiblingDB('app_database');

// Create collections with validation schemas
db.createCollection('user_photos', {
  validator: {
    $jsonSchema: {
      bsonType: 'object',
      required: ['user_id', 'photo_type', 'photo_url'],
      properties: {
        _id: {
          bsonType: 'objectId'
        },
        user_id: {
          bsonType: 'string',
          description: 'User ID is required'
        },
        photo_type: {
          bsonType: 'string',
          enum: ['profile', 'emirates_id', 'verification'],
          description: 'Photo type is required'
        },
        photo_url: {
          bsonType: 'string',
          description: 'Photo URL is required'
        },
        is_verified: {
          bsonType: 'bool'
        },
        created_at: {
          bsonType: 'date'
        },
        updated_at: {
          bsonType: 'date'
        }
      }
    }
  }
});

db.createCollection('users', {
  validator: {
    $jsonSchema: {
      bsonType: 'object',
      required: ['email', 'password_hash', 'phone', 'country_code', 'first_name', 'last_name', 'role'],
      properties: {
        _id: {
          bsonType: 'objectId'
        },
        email: {
          bsonType: 'string',
          pattern: '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$',
          description: 'Email must be a valid email address'
        },
        password_hash: {
          bsonType: 'string',
          description: 'Password hash is required'
        },
        phone: {
          bsonType: 'string',
          maxLength: 20,
          description: 'Phone number is required'
        },
        country_code: {
          bsonType: 'string',
          maxLength: 10,
          description: 'Country code is required'
        },
        first_name: {
          bsonType: 'string',
          maxLength: 100,
          description: 'First name is required'
        },
        last_name: {
          bsonType: 'string',
          maxLength: 100,
          description: 'Last name is required'
        },
        role: {
          bsonType: 'string',
          enum: ['user', 'admin'],
          description: 'User role is required'
        },
        is_active: {
          bsonType: 'bool'
        },
        email_verified: {
          bsonType: 'bool'
        },
        phone_verified: {
          bsonType: 'bool'
        },
        photos: {
          bsonType: 'array',
          items: {
            bsonType: 'object',
            required: ['photo_type', 'photo_url'],
            properties: {
              photo_type: {
                bsonType: 'string',
                enum: ['profile', 'emirates_id', 'verification']
              },
              photo_url: {
                bsonType: 'string'
              },
              is_verified: {
                bsonType: 'bool'
              },
              created_at: {
                bsonType: 'date'
              },
              updated_at: {
                bsonType: 'date'
              }
            }
          }
        },
        preferences: {
          bsonType: 'object',
          properties: {
            theme: { bsonType: 'string', enum: ['light', 'dark'] },
            language: { bsonType: 'string' },
            notifications: { bsonType: 'bool' }
          }
        },
        created_at: {
          bsonType: 'date'
        },
        updated_at: {
          bsonType: 'date'
        }
      }
    }
  }
});



// Create sessions collection for user sessions
db.createCollection('sessions', {
  validator: {
    $jsonSchema: {
      bsonType: 'object',
      required: ['user_id', 'session_token', 'expires_at'],
      properties: {
        _id: {
          bsonType: 'objectId'
        },
        user_id: {
          bsonType: 'objectId'
        },
        session_token: {
          bsonType: 'string'
        },
        refresh_token: {
          bsonType: 'string'
        },
        ip_address: {
          bsonType: 'string'
        },
        user_agent: {
          bsonType: 'string'
        },
        expires_at: {
          bsonType: 'date'
        },
        created_at: {
          bsonType: 'date'
        }
      }
    }
  }
});

// Create audit_logs collection
db.createCollection('audit_logs', {
  validator: {
    $jsonSchema: {
      bsonType: 'object',
      required: ['collection_name', 'operation', 'timestamp'],
      properties: {
        _id: {
          bsonType: 'objectId'
        },
        collection_name: {
          bsonType: 'string'
        },
        operation: {
          bsonType: 'string',
          enum: ['insert', 'update', 'delete']
        },
        document_id: {
          bsonType: 'objectId'
        },
        old_values: {
          bsonType: 'object'
        },
        new_values: {
          bsonType: 'object'
        },
        user_id: {
          bsonType: 'objectId'
        },
        timestamp: {
          bsonType: 'date'
        }
      }
    }
  }
});

// Create indexes for better performance
db.users.createIndex({ "username": 1 }, { unique: true });
db.users.createIndex({ "email": 1 }, { unique: true });
db.users.createIndex({ "created_at": -1 });
db.users.createIndex({ "is_active": 1 });

// Create indexes for user_photos collection
db.user_photos.createIndex({ "user_id": 1 });
db.user_photos.createIndex({ "photo_type": 1 });
db.user_photos.createIndex({ "created_at": -1 });

db.sessions.createIndex({ "user_id": 1 });
db.sessions.createIndex({ "session_token": 1 }, { unique: true });
db.sessions.createIndex({ "expires_at": 1 }, { expireAfterSeconds: 0 });

db.audit_logs.createIndex({ "collection_name": 1 });
db.audit_logs.createIndex({ "timestamp": -1 });
db.audit_logs.createIndex({ "user_id": 1 });

// Insert sample data for development
const sampleUsers = [
  {
    email: 'admin@app.com',
    password_hash: '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.s5uO.G',
    phone: '+971501234567',
    country_code: '+971',
    first_name: 'Admin',
    last_name: 'User',
    role: 'admin',
    is_active: true,
    email_verified: true,
    phone_verified: true,
    photos: [],
    preferences: {
      theme: 'dark',
      language: 'en',
      notifications: true
    },
    created_at: new Date(),
    updated_at: new Date()
  },
  {
    email: 'test@app.com',
    password_hash: '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.s5uO.G',
    phone: '+971507654321',
    country_code: '+971',
    first_name: 'Test',
    last_name: 'User',
    role: 'user',
    is_active: true,
    email_verified: true,
    phone_verified: false,
    photos: [],
    preferences: {
      theme: 'light',
      language: 'en',
      notifications: false
    },
    created_at: new Date(),
    updated_at: new Date()
  }
];

// Insert users if they don't exist
sampleUsers.forEach(user => {
  const existingUser = db.users.findOne({ email: user.email });
  if (!existingUser) {
    const result = db.users.insertOne(user);
    print(`Inserted user: ${user.email} with ID: ${result.insertedId}`);
  } else {
    print(`User ${user.email} already exists`);
  }
});

// Create a user for the application to use
db.createUser({
  user: 'app_user',
  pwd: 'app_password',
  roles: [
    {
      role: 'readWrite',
      db: 'app_database'
    }
  ]
});

print('MongoDB initialization completed successfully!');
print('Collections created: user_photos, users, sessions, audit_logs');
print('Indexes created for optimal performance');
print('Sample data inserted for development');
print('Application user created: app_user');
