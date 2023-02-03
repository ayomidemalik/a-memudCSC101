--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email character varying(50) NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL,
    c_mobile character varying(15)
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, eid, data_id, c_mobile) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	102	5	8055089112
111	Lilian Jaiya	43	l_jaiya@gmail.com	100	3	8055185341
112	Arthur Musa	50	a_musa@gmail.com	107	10	7055282813
113	Philip Akonjo	41	p_akonjo@gmail.com	100	2	9052356772
114	Marylene Mapa	33	m_mapa@gmail.com	120	5	8053333551
115	Oghenero Agor	50	o_agor@gmail.com	117	11	7055566774
116	Adams Bree	33	a_bree@gmail.com	102	1	8056765424
117	Okafor Mathias	45	o_mathias@gmail.com	120	10	8056763367
118	Samson Adeleke	65	s_adeleke@gmail.com	117	11	7056774423
119	Lawal Tamire	35	l_tamire@gmail.com	107	5	9052111101
120	James Job	44	j_job@gmail.com	100	8	8059693919
121	Matthew Jakande	21	m_jakande@gmail.com	120	2	7051232144
122	Jimila Adegboye	20	j_adegboye@gmail.com	107	5	8054921923
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

