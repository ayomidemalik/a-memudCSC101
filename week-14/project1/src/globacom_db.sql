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
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character varying(50) NOT NULL,
    data_duration_in_days integer NOT NULL,
    data_price_in_naira integer
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation character(50),
    pno integer
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    project_managerid integer NOT NULL,
    pname text NOT NULL,
    pno integer,
    pduration character(50)
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    dno integer NOT NULL,
    staff_sal integer,
    age integer,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

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
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration_in_days, data_price_in_naira) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	Administration	Ikeja                                             	44
101	2	Account	Egbeda                                            	11
100	3	Packaging	Ajah                                              	44
120	4	Research	V.I                                               	33
97	5	Account	Magodo                                            	22
122	6	Operations	Mile 2                                            	44
107	7	Packaging	Ketu                                              	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (project_managerid, pname, pno, pduration) FROM stdin;
102	A	11	9 Months                                          
97	B	22	14 Months                                         
120	C	33	16 Months                                         
108	D	44	25 Months                                         
107	E	55	9 Months                                          
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
100	Mustapha Ali	3	175000	32	8063285831
107	Alokwe Martin	7	380000	48	7090082812
97	Dankade Aminat	5	550000	40	9023688832
108	Josiah Joshua	1	120000	30	8053189131
102	Mankinde Mary	2	450000	55	9023487830
120	Adeleke Jane	4	200000	38	7061045862
122	Osahon Mark	6	320000	44	8022289842
104	Kuti Lawal	1	750000	35	9145689842
117	Suleman Ajayi	3	800000	50	7030089981
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (project_managerid);


--
-- PostgreSQL database dump complete
--

