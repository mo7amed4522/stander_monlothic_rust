// MongoDB initialization script for development environment

// Switch to the application database
db = db.getSiblingDB('stander_db');

// Create collections with validation schemas
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

// Create examples collection
db.createCollection('examples', {
  validator: {
    $jsonSchema: {
      bsonType: 'object',
      required: ['title', 'user_id'],
      properties: {
        _id: {
          bsonType: 'objectId'
        },
        title: {
          bsonType: 'string',
          minLength: 1,
          maxLength: 255,
          description: 'Title is required and must be between 1-255 characters'
        },
        description: {
          bsonType: 'string',
          maxLength: 1000
        },
        content: {
          bsonType: 'object',
          description: 'Content can be any valid JSON object'
        },
        user_id: {
          bsonType: 'objectId',
          description: 'User ID is required'
        },
        tags: {
          bsonType: 'array',
          items: {
            bsonType: 'string'
          }
        },
        metadata: {
          bsonType: 'object',
          properties: {
            category: { bsonType: 'string' },
            priority: { bsonType: 'string', enum: ['low', 'medium', 'high'] },
            status: { bsonType: 'string', enum: ['draft', 'published', 'archived'] }
          }
        },
        is_active: {
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

db.examples.createIndex({ "user_id": 1 });
db.examples.createIndex({ "title": "text", "description": "text" });
db.examples.createIndex({ "created_at": -1 });
db.examples.createIndex({ "tags": 1 });
db.examples.createIndex({ "metadata.category": 1 });
db.examples.createIndex({ "metadata.status": 1 });

db.sessions.createIndex({ "user_id": 1 });
db.sessions.createIndex({ "session_token": 1 }, { unique: true });
db.sessions.createIndex({ "expires_at": 1 }, { expireAfterSeconds: 0 });

db.audit_logs.createIndex({ "collection_name": 1 });
db.audit_logs.createIndex({ "timestamp": -1 });
db.audit_logs.createIndex({ "user_id": 1 });

// Insert sample data for development
const sampleUsers = [
  {
    username: 'admin',
    email: 'admin@stander.com',
    password_hash: '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.s5uO.G',
    first_name: 'Admin',
    last_name: 'User',
    is_active: true,
    is_verified: true,
    profile: {
      bio: 'System Administrator',
      location: 'Server Room'
    },
    preferences: {
      theme: 'dark',
      language: 'en',
      notifications: true
    },
    created_at: new Date(),
    updated_at: new Date()
  },
  {
    username: 'testuser',
    email: 'test@stander.com',
    password_hash: '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.s5uO.G',
    first_name: 'Test',
    last_name: 'User',
    is_active: true,
    is_verified: true,
    profile: {
      bio: 'Test user for development',
      location: 'Development Environment'
    },
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
    print(`Inserted user: ${user.username} with ID: ${result.insertedId}`);
  } else {
    print(`User ${user.username} already exists`);
  }
});

// Get admin user ID for sample examples
const adminUser = db.users.findOne({ username: 'admin' });
const testUser = db.users.findOne({ username: 'testuser' });

if (adminUser && testUser) {
  const sampleExamples = [
    {
      title: 'MongoDB Sample Document',
      description: 'This is a sample document stored in MongoDB',
      content: {
        type: 'document',
        format: 'json',
        data: {
          message: 'Hello from MongoDB!',
          features: ['NoSQL', 'Document Store', 'Flexible Schema']
        }
      },
      user_id: adminUser._id,
      tags: ['mongodb', 'nosql', 'sample'],
      metadata: {
        category: 'database',
        priority: 'medium',
        status: 'published'
      },
      is_active: true,
      created_at: new Date(),
      updated_at: new Date()
    },
    {
      title: 'Development Example',
      description: 'Example document for development and testing',
      content: {
        type: 'test',
        environment: 'development',
        config: {
          debug: true,
          logging: 'verbose'
        }
      },
      user_id: testUser._id,
      tags: ['development', 'testing', 'config'],
      metadata: {
        category: 'development',
        priority: 'low',
        status: 'draft'
      },
      is_active: true,
      created_at: new Date(),
      updated_at: new Date()
    }
  ];

  // Insert examples if they don't exist
  sampleExamples.forEach(example => {
    const existingExample = db.examples.findOne({ title: example.title });
    if (!existingExample) {
      const result = db.examples.insertOne(example);
      print(`Inserted example: ${example.title} with ID: ${result.insertedId}`);
    } else {
      print(`Example ${example.title} already exists`);
    }
  });
}

// Create a user for the application to use
db.createUser({
  user: 'stander_app',
  pwd: 'stander_password',
  roles: [
    {
      role: 'readWrite',
      db: 'stander_db'
    }
  ]
});

print('MongoDB initialization completed successfully!');
print('Collections created: users, examples, sessions, audit_logs');
print('Indexes created for optimal performance');
print('Sample data inserted for development');
print('Application user created: stander_app');
