CREATE SEQUENCE IF NOT EXISTS public.availability_id_seq
    INCREMENT 1
    START 1
    MINVALUE 1
    MAXVALUE 9223372036854775807
    CACHE 1;

CREATE TABLE IF NOT EXISTS public.availability
(
    id bigint NOT NULL DEFAULT nextval('availability_id_seq'::regclass),
    ip_or_domain text COLLATE pg_catalog."default" NOT NULL,
    status_code smallint NOT NULL,
    "timestamp" timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT availability_pkey PRIMARY KEY (id)
);