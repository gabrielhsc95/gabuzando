use yew::prelude::*;
use crate::components::window::WindowProps;

pub fn get_cv_windows() -> Vec<WindowProps> {
    vec![
        // Experience (Cells 1, 4 - Left Column)
        WindowProps {
            title: AttrValue::from("cv/experience"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Quantitative Researcher (Senior Associate)"}</b></div>
                            <div>{"Aug. 2024 – Present"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States"}</p>
                        <ul>
                            <li>{"Developed an AI financial analyst agent, focusing on named entity recognition and validation."}</li>
                            <li>{"Participated in research discussions to enhance agent capabilities using various LLM models."}</li>
                            <li>{"Understood client engagement to improve on pain points."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Senior Associate)"}</b></div>
                            <div>{"Jan. 2024 - Jul. 2024"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States"}</p>
                        <ul>
                            <li>{"Teach Led in Testing, empowering 4 people to implement new features to an automated testing software"}</li>
                            <li>{"Managed the testing side implementation of two new pricing models."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Associate)"}</b></div>
                            <div>{"Jan. 2023 - Dec. 2023"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States"}</p>
                        <ul>
                            <li>{"Analytical Quality Assurance for financial factor models."}</li>
                            <li>{"Developed and reviewed financial building blocks libraries to facilitate the replication of financial models."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Analyst)"}</b></div>
                            <div>{"Jun. 2021 - Dec. 2022"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Norman, Oklahoma, United States and Budapest, Hungary"}</p>
                        <ul>
                            <li>{"Built an automated testing software for a financial pricing analytics library, and integrated it to the build pipeline."}</li>
                            <li>{"Developed two financial models to estimate financed emissions in accordance with PCAF."}</li>
                        </ul>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Financial Engineer (Intern)"}</b></div>
                            <div>{"Sep. 2020 - Jun. 2021"}</div>
                        </div>
                        <p><i>{"MSCI"}</i></p>
                        <p>{"Budapest, Hungary"}</p>
                        <ul>
                            <li>{"Analytical Quality Assurance for a financial pricing analytics library."}</li>
                            <li>{"Engage with developer and research about new features and bugs."}</li>
                        </ul>
                    </div>
                </>
            }]),
            x: 1.0,
            y: 10.0,
            width: 31.6,
            height: 55.6,
            buffer: 20.0,
        },
        // Education (Cells 2, 3 - Top Row, Center+Right)
        WindowProps {
            title: AttrValue::from("cv/education"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                     <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Physics (MSc)"}</b></div>
                            <div>{"Sep. 2018 – Jun. 2020"}</div>
                        </div>
                        <p><i>{"Eötvös Loránd Tudományegyetem (ELTE)"}</i></p>
                        <p>{"Budapest, Hungary"}</p>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Physics (BSc)"}</b></div>
                            <div>{"Feb. 2012 – Mar. 2017"}</div>
                        </div>
                        <p><i>{"Universidade Estadual de Londrina (UEL)"}</i></p>
                        <p>{"Londrina, Paraná, Brazil"}</p>
                    </div>
                    <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Physics (BSc)"}</b></div>
                            <div>{"Aug. 2014 – May. 2015"}</div>
                        </div>
                        <p><i>{"Xavier University (XU)"}</i></p>
                        <p>{"Cincinnati, Ohio, United States"}</p>
                    </div>
                     <div class="cv-item">
                        <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
                            <div><b>{"Electromechanics (Technical Course)"}</b></div>
                            <div>{"Feb. 2010 – Dec. 2011"}</div>
                        </div>
                        <p><i>{"Serviço Nacional de Aprendizagem Industrial (SENAI)"}</i></p>
                        <p>{"Londrina, Paraná, Brazil"}</p>
                    </div>
                </>
            }]),
            x: 34.6,
            y: 10.0,
            width: 64.2,
            height: 27.3,
            buffer: 20.0,
        },
        // Skills (Cells 5, 6 - Middle Row, Center+Right)
        WindowProps {
            title: AttrValue::from("cv/skills"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <p><b>{"Languages"}</b>{": Native Portuguese, Advanced English, Intermediate Spanish"}</p>
                    <p><b>{"Programming languages"}</b>{": Rust (leptos, clap, derive_more, serde), Python (pandas, numpy, Django, FastAPI, LangChain, LangGraph, Pydantic), Matlab, Mathematica, C, C++, Java"}</p>
                    <p><b>{"Tools"}</b>{": Microsoft Office, Image Editing, Audio Editing, Video Editing"}</p>
                    <p><b>{"Soft Skills"}</b>{": Public Speaking, Teamwork, Leadership, Problem-Solving"}</p>
                </>
            }]),
            x: 34.6,
            y: 39.3,
            width: 64.2,
            height: 26.3,
            buffer: 20.0,
        },
        // Additional Information (Cells 7, 8, 9 - Bottom Row)
        WindowProps {
            title: AttrValue::from("cv/additional_information"),
            content: yew::html::ChildrenRenderer::new(vec![html! {
                <>
                    <div>
                        <b>{"Volunteering"}</b>
                        <p>{"Teaching programming for a countryside kid since 2022."}</p>
                        <p>{"Taught Mathematics to countryside kids in Brazil during the COVID-19 pandemic (2021)."}</p>
                        <p>{"stuff"}</p>
                    </div>
                    <div>
                        <b>{"Awards"}</b>
                        <p>{"Best Student in the Electromechanics Program, 2011"}</p>
                        <p>{"Bronze Medal in the Brazilian Olympics of Astronomy and Astronautics, 2009"}</p>
                        <p>{"Honorable Mention in the Brazilian Olympics of Mathematics for Public Schools, 2007"}</p>
                    </div>
                    <div>
                        <b>{"Seminars"}</b>
                        <p>{"Presented Introduction to Cosmic Topology, Londrina-PR, Brazil, 2016"}</p>
                        <p>{"Presented Physics of Boomerang, Londrina-PR, Brazil, 2013"}</p>
                        <p>{"Attended IV Jayme Tiomno School of Cosmology at National Observatory, Rio de Janeiro-RJ, Brazil, 2016"}</p>
                        <p>{"Attended Physics Week at the State University of Londrina XVI (2011), XVII (2012), XVIII (2013), XX (2015), and XXI (2016)"}</p>
                    </div>
                    <div>
                        <b>{"Certifications"}</b>
                        <p>{"Stipendium Hungaricum by Tempus Public Foundation between Sep. 2018 – Jul. 2020"}</p>
                        <p>{"Tutorial Education Program (PET) by the Brazilian Ministry of Education (MEC) between Mar. 2013 – Mar. 2014  and Oct. 2015 – Mar. 2017"}</p>
                        <p>{"Science Without Borders by Coordination for the Improvement of Higher Education Personnel (CAPES) and National Council for Scientific and Technological Development (CNPq) between Aug. 2014 – May 2015"}</p>
                        <p>{"FIEP Scholarship by Federation of Industries of Paraná State (FIEP) between  Feb. 2010 – Dec. 2011"}</p>
                    </div>
                </>
            }]),
            x: 1.0,
            y: 67.6,
            width: 97.8,
            height: 27.3,
            buffer: 20.0,
        },
    ]
}
