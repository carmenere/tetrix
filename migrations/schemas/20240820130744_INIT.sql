CREATE TABLE IF NOT EXISTS arches (
    id BIGSERIAL NOT NULL,

    name VARCHAR(32) NOT NULL,
    description VARCHAR(64),

    PRIMARY KEY (id),
    UNIQUE (name)
);

CREATE TABLE IF NOT EXISTS vendors (
    id BIGSERIAL NOT NULL,

    name VARCHAR(32) NOT NULL,
    description VARCHAR(64),

    PRIMARY KEY (id),
    UNIQUE (name)
);

CREATE TABLE IF NOT EXISTS hardware (
    id BIGSERIAL NOT NULL,
    
    vendor_id BIGINT NOT NULL,
    arch_id BIGINT NOT NULL,

    PRIMARY KEY (id),
    UNIQUE (vendor_id, arch_id)
);